use weaverail_model::{
    event::EmitEventType,
    model::{
        DiagramRoot,
        template_train::{TemplateTrain, TemplateTrainId},
    },
};

use crate::command::{Command, CommandError, EventEmitter};

/// テンプレート列車追加操作
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AddTemplateTrainCommand {
    template_train: TemplateTrain,
}

impl AddTemplateTrainCommand {
    pub fn new(template_train: TemplateTrain) -> Self {
        Self { template_train }
    }
}

impl Command for AddTemplateTrainCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.add_template_train(self.template_train.clone())?;
        emitter.emit(EmitEventType::TemplateTrainAdded, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.delete_template_train(self.template_train.id)?;
        emitter.emit(EmitEventType::TemplateTrainDeleted, "");
        Ok(())
    }
}

/// テンプレート列車削除操作
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RemoveTemplateTrainCommand {
    template_train_id: TemplateTrainId,
    template_train: Option<TemplateTrain>,
}

impl RemoveTemplateTrainCommand {
    pub fn new(template_train_id: TemplateTrainId) -> Self {
        Self {
            template_train_id,
            template_train: None,
        }
    }
}

impl Command for RemoveTemplateTrainCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let template_train = obj.delete_template_train(self.template_train_id)?;
        self.template_train = Some(template_train);
        emitter.emit(EmitEventType::TemplateTrainDeleted, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        if let Some(template_train) = self.template_train.clone() {
            obj.add_template_train(template_train)?;
        }
        emitter.emit(EmitEventType::TemplateTrainAdded, "");
        Ok(())
    }
}
