use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 列車同士の時刻調整を表す
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct TrainsAdjustment {
    /// 識別ID
    pub id: Uuid,
    /// 時刻調整種別
    pub adjustment: TrainsAdjustmentType,
}
impl TrainsAdjustment {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            ..Default::default()
        }
    }
}

/// 列車同士の時刻調整種別を表す
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum TrainsAdjustmentType {
    /// 時刻調整を行わない
    None,
    /// 待避を行う
    Waiting {
        /// 待避を行う駅ID
        station_id: Uuid,
        /// (変更がある場合)待避列車の番線ID
        waiting_track_id: Option<Uuid>,
        /// 待避側の列車ID
        waiting_train_id: Uuid,
        /// 通過側の列車ID
        passing_train_id: Uuid,
    },
}
impl Default for TrainsAdjustmentType {
    fn default() -> Self {
        Self::None
    }
}
