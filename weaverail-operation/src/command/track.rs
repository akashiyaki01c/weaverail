use weaverail_model::{
    event::EmitEventType,
    model::{DiagramRoot, track::{Track, TrackId}},
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
        Self { track_id, track: None }
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
