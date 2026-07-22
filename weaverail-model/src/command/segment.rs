//! 駅間(`LineSegment`)関係の操作を定義するモジュール
//! 
//! 以下の操作が含まれる
//! - AddSegmentCommand
//! - RemoveSegmentCommand
//! - PushBackSegmentCommand
//! - PushFrontSegmentCommand
//! - *PopBackSegmentCommand*
//! - *PopFrontSegmentCommand*
//! - *ReplaceSegmentCommand*

use crate::{
    command::{Command, CommandError, EventEmitter},
    event::EmitEventType,
    model::{
        DiagramRoot,
        line::LineId,
        line_segment::{LineSegment, LineSegmentId},
    },
};

/// プロジェクトに駅間を追加する操作
/// ID重複時にエラーを返す
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AddSegmentCommand {
    segment: LineSegment,
}
impl AddSegmentCommand {
    pub fn new(segment: LineSegment) -> Self {
        Self { segment }
    }
}
impl Command for AddSegmentCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.add_segment(self.segment.clone())?;
        emitter.emit(EmitEventType::StationAdded, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.delete_segment(self.segment.id)?;
        emitter.emit(EmitEventType::StationDeleted, "");
        Ok(())
    }
}

/// プロジェクトから駅間を削除する操作
/// 対象駅が存在しない場合はエラーを返す
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RemoveSegmentCommand {
    segment_id: LineSegmentId,
    segment: Option<LineSegment>,
}
impl RemoveSegmentCommand {
    pub fn new(segment_id: LineSegmentId) -> Self {
        Self {
            segment_id,
            segment: None,
        }
    }
}
impl Command for RemoveSegmentCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let sta = obj.delete_segment(self.segment_id)?;
        self.segment = Some(sta);
        emitter.emit(EmitEventType::StationDeleted, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        if let Some(segment) = self.segment.clone() {
            obj.add_segment(segment)?;
        }
        emitter.emit(EmitEventType::StationAdded, "");
        Ok(())
    }
}

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
