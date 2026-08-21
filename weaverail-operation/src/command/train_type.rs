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

/// 列車種別名変更操作。
///
/// 変更前の名前を保持するため、`undo`で元の名前へ戻せる。
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RenameTrainTypeCommand {
    train_type_id: TrainTypeId,
    old_name: Option<String>,
    new_name: String,
}

impl RenameTrainTypeCommand {
    pub fn new(train_type_id: TrainTypeId, new_name: &str) -> Self {
        Self {
            train_type_id,
            old_name: None,
            new_name: new_name.to_string(),
        }
    }
}

impl Command for RenameTrainTypeCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let train_type = obj
            .train_types
            .get_mut(&self.train_type_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        self.old_name = Some(train_type.name.clone());
        train_type.name = self.new_name.clone();
        emitter.emit(EmitEventType::TrainTypeAdded, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let train_type = obj
            .train_types
            .get_mut(&self.train_type_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        if let Some(name) = &self.old_name {
            train_type.name = name.clone();
        }
        emitter.emit(EmitEventType::TrainTypeAdded, "");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use weaverail_model::model::id::WeaverailId;

    #[test]
    fn rename_train_type_command_restores_name() {
        let mut root = DiagramRoot::default();
        let id = TrainTypeId::new(WeaverailId::new(1));
        root.add_train_type(TrainType::new(id, "local")).unwrap();
        let mut command = RenameTrainTypeCommand::new(id, "rapid");
        command
            .redo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert_eq!(root.train_types[&id].name, "rapid");
        command
            .undo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert_eq!(root.train_types[&id].name, "local");
    }

    #[test]
    fn rename_train_type_command_handles_missing_type() {
        let mut root = DiagramRoot::default();
        let invalid_id = TrainTypeId::new(WeaverailId::new(99));
        let mut command = RenameTrainTypeCommand::new(invalid_id, "new");
        assert_eq!(
            command.redo(&mut root, &crate::command::EmptyEventEmitter),
            Err(CommandError::TargetObjectNotFound)
        );
    }
}
