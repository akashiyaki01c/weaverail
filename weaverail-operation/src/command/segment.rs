use weaverail_model::{
    event::EmitEventType,
    model::{
        DiagramRoot,
        line::LineId,
        line_segment::{LineSegment, LineSegmentId},
    },
};

use crate::command::{Command, CommandError, EventEmitter};

/// 路線に駅間を追加する操作
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
        emitter.emit(EmitEventType::SegmentPushed, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.delete_segment(self.segment.id)?;
        emitter.emit(EmitEventType::SegmentPoped, "");
        Ok(())
    }
}

/// プロジェクトから駅間を削除する操作
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
        let segment = obj.delete_segment(self.segment_id)?;
        self.segment = Some(segment);
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

/// 路線の末尾の駅間を削除する操作。
///
/// 削除した参照を保持し、`undo`で同じ向きの参照を末尾へ戻す。
#[derive(Clone, PartialEq, Debug, Default)]
pub struct PopBackSegmentCommand {
    line_id: LineId,
    segment: Option<weaverail_model::model::line::SegmentRef>,
}

impl PopBackSegmentCommand {
    /// 指定路線の末尾を削除するコマンドを生成する。
    pub fn new(line_id: LineId) -> Self {
        Self {
            line_id,
            segment: None,
        }
    }
}

impl Command for PopBackSegmentCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        self.segment = Some(obj.pop_back_line_segment(self.line_id)?);
        emitter.emit(EmitEventType::SegmentPoped, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let segment = self.segment.take().ok_or(CommandError::Inconsistent)?;
        obj.push_back_line_segment(self.line_id, segment.segment_id, segment.is_reversed)?;
        self.segment = Some(segment);
        emitter.emit(EmitEventType::SegmentPushed, "");
        Ok(())
    }
}

/// 路線の先頭の駅間を削除する操作。
///
/// 削除した参照を保持し、`undo`で同じ向きの参照を先頭へ戻す。
#[derive(Clone, PartialEq, Debug, Default)]
pub struct PopFrontSegmentCommand {
    line_id: LineId,
    segment: Option<weaverail_model::model::line::SegmentRef>,
}

/// 路線上の駅間を複数の駅間に置換する操作。
///
/// `target_segment_id`を`replacements`の順序で置換し、元の参照を`undo`用に保持する。
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ReplaceSegmentCommand {
    line_id: LineId,
    target_segment_id: LineSegmentId,
    replacements: Vec<weaverail_model::model::line::SegmentRef>,
    original: Option<weaverail_model::model::line::SegmentRef>,
}

impl ReplaceSegmentCommand {
    pub fn new(
        line_id: LineId,
        target_segment_id: LineSegmentId,
        replacements: Vec<weaverail_model::model::line::SegmentRef>,
    ) -> Self {
        Self {
            line_id,
            target_segment_id,
            replacements,
            original: None,
        }
    }
}

impl Command for ReplaceSegmentCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let line = obj
            .lines
            .get_mut(&self.line_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        let index = line
            .segments
            .iter()
            .position(|segment| segment.segment_id == self.target_segment_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        let original = line.segments[index].clone();
        if self.replacements.is_empty() {
            return Err(CommandError::Inconsistent);
        }
        line.segments
            .splice(index..=index, self.replacements.clone());
        self.original = Some(original);
        emitter.emit(EmitEventType::SegmentPushed, "");
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
        let index = line
            .segments
            .iter()
            .position(|segment| {
                self.replacements
                    .iter()
                    .any(|replacement| replacement == segment)
            })
            .ok_or(CommandError::Inconsistent)?;
        let original = self.original.clone().ok_or(CommandError::Inconsistent)?;
        line.segments
            .splice(index..index + self.replacements.len(), [original]);
        emitter.emit(EmitEventType::SegmentPoped, "");
        Ok(())
    }
}

impl PopFrontSegmentCommand {
    pub fn new(line_id: LineId) -> Self {
        Self {
            line_id,
            segment: None,
        }
    }
}

impl Command for PopFrontSegmentCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        self.segment = Some(obj.pop_front_line_segment(self.line_id)?);
        emitter.emit(EmitEventType::SegmentPoped, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let segment = self.segment.take().ok_or(CommandError::Inconsistent)?;
        obj.push_front_line_segment(self.line_id, segment.segment_id, segment.is_reversed)?;
        self.segment = Some(segment);
        emitter.emit(EmitEventType::SegmentPushed, "");
        Ok(())
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use weaverail_model::model::{
        id::WeaverailId,
        line::{Line, SegmentRef},
        line_segment::LineSegmentId,
        station::{Station, StationId},
    };

    #[test]
    fn segment_commands_cover_remove_and_push_operations() {
        let mut root = DiagramRoot::default();
        let start = StationId::new(WeaverailId::new(10));
        let end = StationId::new(WeaverailId::new(11));
        let line_id = LineId::new(WeaverailId::new(20));
        let segment_id = LineSegmentId::new(WeaverailId::new(30));

        root.add_station(Station::new(start, "梅田")).unwrap();
        root.add_station(Station::new(end, "大阪")).unwrap();
        root.add_line(Line::new(line_id, "神明線", &[])).unwrap();

        let segment = LineSegment::new(segment_id, start, end);
        let mut add = AddSegmentCommand::new(segment.clone());
        let mut push = PushBackSegmentCommand::new(line_id, segment.id, false);
        assert!(
            add.redo(&mut root, &crate::command::EmptyEventEmitter)
                .is_ok()
        );
        assert!(
            push.redo(&mut root, &crate::command::EmptyEventEmitter)
                .is_ok()
        );

        let mut remove = RemoveSegmentCommand::new(segment_id);
        assert!(matches!(
            remove.redo(&mut root, &crate::command::EmptyEventEmitter),
            Err(CommandError::ModelError(
                weaverail_model::error::ModelError::ExternalReferenced
            ))
        ));

        assert!(root.pop_back_line_segment(line_id).is_ok());
        assert!(
            remove
                .redo(&mut root, &crate::command::EmptyEventEmitter)
                .is_ok()
        );
        assert!(
            remove
                .undo(&mut root, &crate::command::EmptyEventEmitter)
                .is_ok()
        );

        let mut push_front = PushFrontSegmentCommand::new(line_id, segment.id, false);
        assert!(
            push_front
                .redo(&mut root, &crate::command::EmptyEventEmitter)
                .is_ok()
        );
        assert!(
            push_front
                .undo(&mut root, &crate::command::EmptyEventEmitter)
                .is_ok()
        );
    }

    #[test]
    fn pop_and_replace_segment_commands_restore_line() {
        let mut root = DiagramRoot::default();
        let start = StationId::new(WeaverailId::new(40));
        let end = StationId::new(WeaverailId::new(41));
        let line_id = LineId::new(WeaverailId::new(42));
        let target_id = LineSegmentId::new(WeaverailId::new(43));
        let replacement_id = LineSegmentId::new(WeaverailId::new(44));
        root.add_station(Station::new(start, "A")).unwrap();
        root.add_station(Station::new(end, "B")).unwrap();
        root.add_segment(LineSegment::new(target_id, start, end))
            .unwrap();
        root.add_segment(LineSegment::new(replacement_id, start, end))
            .unwrap();
        root.add_line(Line::new(
            line_id,
            "line",
            &[SegmentRef {
                segment_id: target_id,
                is_reversed: false,
            }],
        ))
        .unwrap();

        let mut pop = PopBackSegmentCommand::new(line_id);
        pop.redo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert!(root.lines[&line_id].segments.is_empty());
        pop.undo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert_eq!(root.lines[&line_id].segments[0].segment_id, target_id);

        let mut replace = ReplaceSegmentCommand::new(
            line_id,
            target_id,
            vec![SegmentRef {
                segment_id: replacement_id,
                is_reversed: true,
            }],
        );
        replace
            .redo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert_eq!(root.lines[&line_id].segments[0].segment_id, replacement_id);
        replace
            .undo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert_eq!(root.lines[&line_id].segments[0].segment_id, target_id);
    }

    #[test]
    fn segment_commands_handle_errors() {
        let mut root = DiagramRoot::default();
        let line_id = LineId::new(WeaverailId::new(50));

        let mut pop_empty = PopBackSegmentCommand::new(line_id);
        assert_eq!(
            pop_empty.redo(&mut root, &crate::command::EmptyEventEmitter),
            Err(CommandError::ModelError(
                weaverail_model::error::ModelError::ObjectNotFound
            ))
        );

        let invalid_line = LineId::new(WeaverailId::new(51));
        let mut replace = ReplaceSegmentCommand::new(
            invalid_line,
            LineSegmentId::new(WeaverailId::new(52)),
            vec![],
        );
        assert_eq!(
            replace.redo(&mut root, &crate::command::EmptyEventEmitter),
            Err(CommandError::TargetObjectNotFound)
        );
    }
}
