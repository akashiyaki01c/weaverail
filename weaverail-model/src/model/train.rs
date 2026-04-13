//! Weaverail上の「列車」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
//! - Train (列車)
//!   - TemplateSegment (テンプレート列車への部分参照)

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{model::{ExtensionProperty, station::StationId, template_train::TemplateTrainId, time::Time}, weaverail_id};

weaverail_id!(TrainId, "TRAI");

/// Weaverail上の1つの「列車」を表す
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Train {
    /// 識別ID
    pub id: TrainId,
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
            id: TrainId::new(),
            ..Default::default()
        }
    }
}

/// Weaverail上のテンプレート列車への部分参照
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct TemplateSegment {
    /// テンプレート列車ID
    pub template_train_id: TemplateTrainId,
    /// 開始駅ID
    pub start_station_id: StationId,
    /// 終了駅ID
    pub end_station_id: StationId,
}
