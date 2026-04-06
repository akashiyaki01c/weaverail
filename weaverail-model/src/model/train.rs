use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::{ExtensionProperty, time::Time};

/// 一つの列車を表す
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Train {
    /// 識別ID
    pub id: Uuid,
    /// テンプレート列車ID
    pub template_segments: Vec<TemplateSegment>,
    /// 開始駅の出発時刻
    pub start_departure_time: Time,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl Train {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            ..Default::default()
        }
    }
}

/// 一つのテンプレート列車への参照を表す
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct TemplateSegment {
    /// テンプレート列車ID
    pub template_train_id: Uuid,
    /// 開始駅ID
    pub start_station_id: Uuid,
    /// 終了駅ID
    pub end_station_id: Uuid,
}
