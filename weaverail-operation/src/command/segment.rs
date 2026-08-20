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
        line::Line,
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
}
