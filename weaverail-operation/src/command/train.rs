use weaverail_model::{
    event::EmitEventType,
    model::{DiagramRoot, timetable::TimetableId, train::Train},
};

use crate::command::{Command, CommandError, EventEmitter};

/// 列車追加操作
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AddTrainCommand {
    timetable_id: TimetableId,
    train: Train,
}

impl AddTrainCommand {
    pub fn new(timetable_id: TimetableId, train: Train) -> Self {
        Self {
            timetable_id,
            train,
        }
    }
}

impl Command for AddTrainCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.add_train(self.train.clone())?;
        emitter.emit(EmitEventType::TrainAdded, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.delete_train(self.train.id)?;
        emitter.emit(EmitEventType::TrainDeleted, "");
        Ok(())
    }
}
