use crate::{
    command::{Command, CommandError, EventEmitter},
    model::{
        DiagramRoot,
        station::{StationId, Track},
    },
};

/// 駅に番線を追加する操作
/// 指定駅が存在しない場合エラー
/// IDが重複した場合エラー
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AddTrackCommand {
    station_id: StationId,
    track: Track,
}
impl AddTrackCommand {
    pub fn new(station_id: StationId, track: Track) -> Self {
        Self { station_id, track }
    }
}
impl Command for AddTrackCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.add_track(self.station_id, self.track.clone())?;
        emitter.emit("track::added", "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.delete_track(self.station_id, self.track.id)?;
        emitter.emit("track::deleted", "");
        Ok(())
    }
}
