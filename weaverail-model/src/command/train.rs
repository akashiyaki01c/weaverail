use crate::{
    command::{Command, CommandError, EventEmitter},
    model::{DiagramRoot, timetable::TimetableId, train::Train},
};

/// 駅に列車を追加する操作
/// 指定時刻表が存在しない場合エラー
/// IDが重複した場合エラー
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AddTrainCommand {
    timetable_id: TimetableId,
    train: Train,
}
impl AddTrainCommand {
    pub fn new(station_id: TimetableId, track: Train) -> Self {
        Self {
            timetable_id: station_id,
            train: track,
        }
    }
}
impl Command for AddTrainCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.add_train(self.timetable_id, self.train.clone())?;
        emitter.emit("train::added", "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.delete_train(self.timetable_id, self.train.id)?;
        emitter.emit("train::deleted", "");
        Ok(())
    }
}
