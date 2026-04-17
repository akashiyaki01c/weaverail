use serde::{Deserialize, Serialize};

use crate::{
    model::{
        station::{StationId, TrackId},
        train::TrainId,
    },
    weaverail_id,
};

weaverail_id!(TrainsAdjustmentId, "ADJT");

/// 列車同士の時刻調整を表す
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct TrainsAdjustment {
    /// 識別ID
    pub id: TrainsAdjustmentId,
    /// 時刻調整種別
    pub adjustment: TrainsAdjustmentType,
}
impl TrainsAdjustment {
    pub fn new(id: TrainsAdjustmentId) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

/// 列車同士の時刻調整種別を表す
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum TrainsAdjustmentType {
    /// 時刻調整を行わない
    #[default]
    None,
    /// 待避を行う
    Waiting {
        /// 待避を行う駅ID
        station_id: StationId,
        /// (変更がある場合)待避列車の番線ID
        waiting_track_id: Option<TrackId>,
        /// 待避側の列車ID
        waiting_train_id: TrainId,
        /// 通過側の列車ID
        passing_train_id: TrainId,
    },
}
