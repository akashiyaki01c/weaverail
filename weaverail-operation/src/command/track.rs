use weaverail_model::{
    event::EmitEventType,
    model::{
        DiagramRoot,
        track::{Track, TrackId},
    },
};

use crate::command::{Command, CommandError, EventEmitter};

/// 番線追加操作
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AddTrackCommand {
    track: Track,
}

impl AddTrackCommand {
    pub fn new(track: Track) -> Self {
        Self { track }
    }
}

impl Command for AddTrackCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.add_track(self.track.clone())?;
        emitter.emit(EmitEventType::TrackAdded, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.delete_track(self.track.id)?;
        emitter.emit(EmitEventType::TrackDeleted, "");
        Ok(())
    }
}

/// 番線削除操作
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RemoveTrackCommand {
    track_id: TrackId,
    track: Option<Track>,
}

impl RemoveTrackCommand {
    pub fn new(track_id: TrackId) -> Self {
        Self {
            track_id,
            track: None,
        }
    }
}

impl Command for RemoveTrackCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let track = obj.delete_track(self.track_id)?;
        self.track = Some(track);
        emitter.emit(EmitEventType::TrackDeleted, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        if let Some(track) = self.track.clone() {
            obj.add_track(track)?;
        }
        emitter.emit(EmitEventType::TrackAdded, "");
        Ok(())
    }
}

/// 番線名変更操作。
///
/// 変更前の名前を保持するため、`undo`で元の名前へ戻せる。
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RenameTrackCommand {
    track_id: TrackId,
    old_name: Option<String>,
    new_name: String,
}

impl RenameTrackCommand {
    pub fn new(track_id: TrackId, new_name: &str) -> Self {
        Self {
            track_id,
            old_name: None,
            new_name: new_name.to_string(),
        }
    }
}

impl Command for RenameTrackCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let track = obj
            .tracks
            .get_mut(&self.track_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        self.old_name = Some(track.name.clone());
        track.name = self.new_name.clone();
        emitter.emit(EmitEventType::TrackAdded, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let track = obj
            .tracks
            .get_mut(&self.track_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        if let Some(name) = &self.old_name {
            track.name = name.clone();
        }
        emitter.emit(EmitEventType::TrackAdded, "");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use weaverail_model::model::{
        id::WeaverailId,
        station::{Station, StationId},
    };

    #[test]
    fn rename_track_command_restores_name() {
        let mut root = DiagramRoot::default();
        let station_id = StationId::new(WeaverailId::new(1));
        let track_id = TrackId::new(WeaverailId::new(2));
        root.add_station(Station::new(station_id, "A")).unwrap();
        root.add_track(Track::new(track_id, station_id, "1"))
            .unwrap();
        let mut command = RenameTrackCommand::new(track_id, "2");
        command
            .redo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert_eq!(root.tracks[&track_id].name, "2");
        command
            .undo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert_eq!(root.tracks[&track_id].name, "1");
    }

    #[test]
    fn rename_track_command_handles_missing_track() {
        let mut root = DiagramRoot::default();
        let invalid_id = TrackId::new(WeaverailId::new(99));
        let mut command = RenameTrackCommand::new(invalid_id, "new");
        assert_eq!(
            command.redo(&mut root, &crate::command::EmptyEventEmitter),
            Err(CommandError::TargetObjectNotFound)
        );
    }
}
