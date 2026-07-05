use serde::{Deserialize, Serialize};

use crate::model::{line_segment::LineSegmentId, station::StationId};

#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ResultWarpCoords {
    pub upper_y: f64,
    pub lower_y: f64,
    pub segment_id: LineSegmentId,
    pub is_reversed: bool,
}

#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ResultWarpStations {
    pub y_coord: f64,
    pub station_id: StationId,
    pub name: String,
}
