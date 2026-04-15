use serde::{Deserialize, Serialize};

use crate::model::{line::LineSegmentId, train::TrainId};

/// 駅間での列車の順序を表す
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct SegmentTrainOrder {
    pub segment_id: LineSegmentId,
    pub is_reversed: bool,
    pub order: Vec<TrainId>,
}
