use weaverail_model::{
    event::EmitEventType,
    model::DiagramRoot,
};

use super::{Command, CommandError, EventEmitter};

/// スクリプト実行全体を単一のUndo/Redo操作として記録するコマンド
#[derive(Clone, Debug)]
pub struct ScriptCommand {
    old_root_json: Vec<u8>,
    new_root_json: Vec<u8>,
}

impl ScriptCommand {
    pub fn new(old_root: DiagramRoot, new_root: DiagramRoot) -> Self {
        Self {
            old_root_json: serde_json::to_vec(&old_root).expect("DiagramRoot must serialize"),
            new_root_json: serde_json::to_vec(&new_root).expect("DiagramRoot must serialize"),
        }
    }
}

impl Command for ScriptCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        *obj = serde_json::from_slice(&self.new_root_json)
            .map_err(|_| CommandError::Inconsistent)?;
        emitter.emit(EmitEventType::ScriptExecuted, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        *obj = serde_json::from_slice(&self.old_root_json)
            .map_err(|_| CommandError::Inconsistent)?;
        emitter.emit(EmitEventType::ScriptExecuted, "");
        Ok(())
    }
}
