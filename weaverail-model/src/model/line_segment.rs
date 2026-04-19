use serde::{Deserialize, Serialize};

use crate::{model::{ExtensionProperty, station::StationId}, weaverail_id};

weaverail_id!(LineSegmentId, "SGM_");

/// Weaverail上の1つの路線に属する駅間を表す構造体
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct LineSegment {
    /// 識別ID
    pub id: LineSegmentId,
    /// 開始駅ID
    pub start_station: StationId,
    /// 終了駅ID
    pub end_station: StationId,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl LineSegment {
    pub fn new(id: LineSegmentId, start_station: StationId, end_station: StationId) -> Self {
        Self {
            id,
            start_station,
            end_station,
            ..Default::default()
        }
    }

    /// 駅間が指定駅を参照しているか
    pub fn contains_station(&self, station_id: StationId) -> bool {
        self.start_station == station_id || self.end_station == station_id
    }
}