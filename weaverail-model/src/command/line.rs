use crate::{
    command::{Command, CommandError, EventEmitter},
    event::EmitEventType,
    model::{
        DiagramRoot,
        line::{Line, LineId},
    },
};

/// プロジェクトに路線を追加する操作
/// ID重複時にエラーを返す
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AddLineCommand {
    line: Line,
}
impl AddLineCommand {
    pub fn new(line: Line) -> Self {
        Self { line }
    }
}
impl Command for AddLineCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), super::CommandError> {
        obj.add_line(self.line.clone())?;
        emitter.emit(EmitEventType::LineAdded, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), super::CommandError> {
        obj.delete_line(self.line.id)?;
        emitter.emit(EmitEventType::LineDeleted, "");

        Ok(())
    }
}

/// プロジェクトから路線を削除する操作
/// 対象路線が存在しない場合はエラーを返す
/// テンプレート列車から参照されている場合はエラーを返す
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RemoveLineCommand {
    line_id: LineId,
    line: Option<Line>,
}
impl RemoveLineCommand {
    pub fn new(line_id: LineId) -> Self {
        Self {
            line_id,
            line: None,
        }
    }
}

impl Command for RemoveLineCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let line = obj.delete_line(self.line_id)?;
        self.line = Some(line);
        emitter.emit(EmitEventType::LineDeleted, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        if let Some(line) = self.line.clone() {
            obj.add_line(line)?;
        }
        emitter.emit(EmitEventType::LineAdded, "");
        Ok(())
    }
}

/// 指定路線の駅名を変更する操作
/// 対象路線が存在しない場合はエラーを返す
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RenameLineCommand {
    line_id: LineId,
    old_name: Option<String>,
    new_name: String,
}
impl RenameLineCommand {
    pub fn new(line_id: LineId, new_name: &str) -> Self {
        Self {
            line_id,
            old_name: None,
            new_name: new_name.to_string(),
        }
    }
}
impl Command for RenameLineCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let line = obj
            .lines
            .get_mut(&self.line_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        self.old_name = Some(line.name.to_string());
        line.name = self.new_name.clone();
        emitter.emit(EmitEventType::LineRenamed, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let line = obj
            .lines
            .get_mut(&self.line_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        if let Some(name) = &self.old_name {
            line.name = name.clone();
        }
        emitter.emit(EmitEventType::LineRenamed, "");
        Ok(())
    }
}
