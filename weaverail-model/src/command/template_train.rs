use crate::{
    command::{Command, CommandError, EventEmitter},
    model::{DiagramRoot, template_train::TemplateTrain},
};

/// プロジェクトにテンプレート列車を追加する操作
/// ID重複時にエラーを返す
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
        emitter.emit("template_train::added", "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.delete_template_train(self.template_train.id)?;
        emitter.emit("template_train::deleted", "");
        Ok(())
    }
}
