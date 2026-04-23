use serde::{Deserialize, Serialize};

use crate::model::{line_segment::LineSegmentId, station::StationId, time::Time, train::TrainId};

#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ResultWeftTrain {
    pub train_id: TrainId,
    pub times: Vec<ResultWeftTime>,
}

#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ResultWeftTime {
    pub train_id: TrainId,
    pub before_segment_id: Option<LineSegmentId>,
    pub next_segment_id: Option<LineSegmentId>,
    pub station_id: StationId,
    pub arrival_time: Option<Time>,
    pub departure_time: Option<Time>,
    pub stop_type: StopType,
}

#[derive(ts_rs::TS, Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum StopType {
    #[default]
    Stop,
    Pass,
}
