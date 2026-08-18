use weaverail_model::{
    event::EmitEventType,
    model::{
        DiagramRoot,
        train_type::{TrainType, TrainTypeId},
    },
};

use crate::command::{Command, CommandError, EventEmitter};

/// 列車種別追加操作
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AddTrainTypeCommand {
    train_type: TrainType,
}

impl AddTrainTypeCommand {
    pub fn new(train_type: TrainType) -> Self {
        Self { train_type }
    }
}

impl Command for AddTrainTypeCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.add_train_type(self.train_type.clone())?;
        emitter.emit(EmitEventType::TrainTypeAdded, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.delete_train_type(self.train_type.id)?;
        emitter.emit(EmitEventType::TrainTypeDeleted, "");
        Ok(())
    }
}

/// 列車種別削除操作
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RemoveTrainTypeCommand {
    train_type_id: TrainTypeId,
    train_type: Option<TrainType>,
}

impl RemoveTrainTypeCommand {
    pub fn new(train_type_id: TrainTypeId) -> Self {
        Self {
            train_type_id,
            train_type: None,
        }
    }
}

impl Command for RemoveTrainTypeCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let train_type = obj.delete_train_type(self.train_type_id)?;
        self.train_type = Some(train_type);
        emitter.emit(EmitEventType::TrainTypeDeleted, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        if let Some(train_type) = self.train_type.clone() {
            obj.add_train_type(train_type)?;
        }
        emitter.emit(EmitEventType::TrainTypeAdded, "");
        Ok(())
    }
}
