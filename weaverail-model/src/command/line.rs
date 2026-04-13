use crate::{
    command::{Command, CommandError, EventEmitter},
    model::{
        DiagramRoot,
        line::{Line, LineSegment},
    },
};
use uuid::Uuid;

/// 路線の追加
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
        emitter.emit("line::added", "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), super::CommandError> {
        obj.delete_line(self.line.id)?;
        emitter.emit("line::removed", "");

        Ok(())
    }
}

/// 路線を削除する
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RemoveLineCommand {
    line_id: Uuid,
    line: Option<Line>,
}
impl RemoveLineCommand {
    pub fn new(line_id: Uuid) -> Self {
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
        emitter.emit("line::removed", "");
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
        emitter.emit("line::added", "");
        Ok(())
    }
}

/// 路線に駅間を追加する
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AppendSegmentToLine {
    line_id: Uuid,
    segment: LineSegment,
}
impl AppendSegmentToLine {
    pub fn new(line_id: Uuid, segment: LineSegment) -> Self {
        Self { line_id, segment }
    }
}

impl Command for AppendSegmentToLine {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.append_segment(self.line_id, self.segment.clone())?;
        emitter.emit("segment::append", "");
        Ok(())
    }

    fn undo(
        &mut self,
        _obj: &mut DiagramRoot,
        _emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        todo!();
    }
}
