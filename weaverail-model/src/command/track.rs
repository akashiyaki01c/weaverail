use crate::{
    command::{Command, CommandError, EventEmitter},
    event::EmitEventType,
    model::{DiagramRoot, track::Track},
};

/// 駅に番線を追加する操作
/// 指定駅が存在しない場合エラー
/// IDが重複した場合エラー
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
