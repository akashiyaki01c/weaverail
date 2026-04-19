use crate::{
    command::{Command, CommandError, EventEmitter},
    event::EmitEventType,
    id_issuer::IdIssuer,
    model::{
        DiagramRoot,
        line::{Line, LineId},
        line_segment::{LineSegment, LineSegmentId},
        station::StationId,
    },
};

/// 路線の末尾に駅間を追加する操作
#[derive(Clone, PartialEq, Debug, Default)]
pub struct PushBackSegmentCommand {
    line_id: LineId,
    segment: LineSegment,
}
impl PushBackSegmentCommand {
    pub fn new(line_id: LineId, segment: LineSegment) -> Self {
        Self { line_id, segment }
    }
    pub fn new_from_station_id(
        root: &DiagramRoot,
        line: &Line,
        station_id: StationId,
        issuer: &mut IdIssuer,
    ) -> Self {
        let segment = root.segments.get(line.segments.last().unwrap()).unwrap();
        let segment = LineSegment::new(
            LineSegmentId::new(issuer.next()),
            segment.end_station,
            station_id,
        );
        Self {
            line_id: line.id,
            segment,
        }
    }
}
impl Command for PushBackSegmentCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let line = obj
            .lines
            .get_mut(&self.line_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        let segment = line.segments.last();
        if segment.is_none() {
            obj.segments.insert(self.segment.id, self.segment.clone());
            line.segments.push(self.segment.id);
            return Ok(());
        }
        let last_segment = obj.segments.get(line.segments.last().unwrap()).unwrap();
        if !line.segments.is_empty() && last_segment.end_station != self.segment.start_station {
            return Err(CommandError::Inconsistent);
        }
        obj.segments.insert(self.segment.id, self.segment.clone());
        line.segments.push(self.segment.id);
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
        let last_segment = obj.segments.get(line.segments.last().unwrap()).unwrap();
        if !line.segments.is_empty() && last_segment.id != self.segment.id {
            return Err(CommandError::Inconsistent);
        }
        line.segments.pop();
        emitter.emit(EmitEventType::SegmentPoped, "");
        Ok(())
    }
}

/// 路線の末尾に駅間を追加する操作
#[derive(Clone, PartialEq, Debug, Default)]
pub struct PushFrontSegmentCommand {
    line_id: LineId,
    segment: LineSegment,
}
impl PushFrontSegmentCommand {
    pub fn new(line_id: LineId, segment: LineSegment) -> Self {
        Self { line_id, segment }
    }
    pub fn new_from_station_id(
        root: &DiagramRoot,
        line: &Line,
        station_id: StationId,
        issuer: &mut IdIssuer,
    ) -> Self {
        let first_segment = root.segments.get(line.segments.first().unwrap()).unwrap();
        let segment = LineSegment::new(
            LineSegmentId::new(issuer.next()),
            station_id,
            first_segment.start_station,
        );
        Self {
            line_id: line.id,
            segment,
        }
    }
}
