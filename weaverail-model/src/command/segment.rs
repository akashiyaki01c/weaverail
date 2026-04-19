use crate::{
    command::{Command, CommandError, EventEmitter},
    event::EmitEventType,
    model::{
        DiagramRoot,
        line::LineId,
        line_segment::LineSegmentId,
    },
};

/// 路線の末尾に駅間を追加する操作
#[derive(Clone, PartialEq, Debug, Default)]
pub struct PushBackSegmentCommand {
    line_id: LineId,
    segment: LineSegmentId,
    is_reversed: bool,
}
impl PushBackSegmentCommand {
    pub fn new(line_id: LineId, segment_id: LineSegmentId, is_reversed: bool) -> Self {
        Self {
            line_id,
            segment: segment_id,
            is_reversed,
        }
    }
}
impl Command for PushBackSegmentCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.push_back_line_segment(self.line_id, self.segment, self.is_reversed)?;
        emitter.emit(EmitEventType::SegmentPushed, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.pop_back_line_segment(self.line_id)?;
        emitter.emit(EmitEventType::SegmentPoped, "");
        Ok(())
    }
}

/// 路線の先頭に駅間を追加する操作
#[derive(Clone, PartialEq, Debug, Default)]
pub struct PushFrontSegmentCommand {
    line_id: LineId,
    segment: LineSegmentId,
    is_reversed: bool,
}
impl PushFrontSegmentCommand {
    pub fn new(line_id: LineId, segment_id: LineSegmentId, is_reversed: bool) -> Self {
        Self {
            line_id,
            segment: segment_id,
            is_reversed,
        }
    }
}
impl Command for PushFrontSegmentCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.push_front_line_segment(self.line_id, self.segment, self.is_reversed)?;
        emitter.emit(EmitEventType::SegmentPushed, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.pop_front_line_segment(self.line_id)?;
        emitter.emit(EmitEventType::SegmentPoped, "");
        Ok(())
    }
}
