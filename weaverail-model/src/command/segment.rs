use crate::{
    command::{Command, CommandError, EventEmitter},
    model::{
        DiagramRoot,
        line::{Line, LineId, LineSegment},
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
    pub fn new_from_station_id(line: &Line, station_id: StationId) -> Self {
        let segment = LineSegment::new(line.segments.last().unwrap().end_station, station_id);
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
        if !line.segments.is_empty() && line.segments.last().unwrap().end_station != self.segment.start_station {
            return Err(CommandError::Inconsistent);
        }
        line.segments.push(self.segment.clone());
        emitter.emit("segment::pushed", "");
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
        if !line.segments.is_empty() && line.segments.last().unwrap().id != self.segment.id {
            return Err(CommandError::Inconsistent);
        }
        line.segments.pop();
        emitter.emit("segment::popped", "");
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
    pub fn new_from_station_id(line: &Line, station_id: StationId) -> Self {
        let segment = LineSegment::new(station_id, line.segments.first().unwrap().start_station);
        Self {
            line_id: line.id,
            segment,
        }
    }
}
