use weaverail_model::model::{
    DiagramRoot,
    diagram_view_settings::{DiagramViewSegment, DiagramViewSettings},
    line_segment::LineSegmentId,
    time::Time,
};

const DEFAULT_BLANK_TIME: Time = Time::new(0, 2, 0);

pub struct ResultWarpCoords {
    pub upper_y: f64,
    pub lower_y: f64,
    pub segment_id: LineSegmentId,
    pub is_reversed: bool,
}

/// ダイヤグラム上のY座標を求める
pub fn warp_coords(root: &DiagramRoot, settings: &DiagramViewSettings) -> Vec<ResultWarpCoords> {
    let mut result = vec![];
    let mut current_y: f64 = 0.0;
    for segment in &settings.segments {
        match segment {
            DiagramViewSegment::Black { scale } => {
                current_y += DEFAULT_BLANK_TIME.total_second() as f64 * *scale as f64;
            }
            DiagramViewSegment::StationBetween { segment } => {
                let upper_y = current_y;
                let segments = root.get_template_segments(segment.segment_id);
                let time = segments
                    .iter()
                    .max_by(|x, y| x.running_time.cmp(&y.running_time));
                let time = if time.is_none() {
                    DEFAULT_BLANK_TIME
                } else {
                    time.unwrap().running_time
                };
                current_y += time.total_second() as f64;
                let lower_y = current_y;

                result.push(ResultWarpCoords {
                    upper_y,
                    lower_y,
                    segment_id: segment.segment_id,
                    is_reversed: segment.is_reversed,
                });
            }
        }
    }
    result
}
