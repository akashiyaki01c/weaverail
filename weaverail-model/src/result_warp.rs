use serde::{Deserialize, Serialize};

use crate::model::line_segment::LineSegmentId;

#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ResultWarpCoords {
    pub upper_y: f64,
    pub lower_y: f64,
    pub segment_id: LineSegmentId,
    pub is_reversed: bool,
}
