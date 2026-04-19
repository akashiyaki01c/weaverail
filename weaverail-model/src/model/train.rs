//! Weaverail上の「列車」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
//! - Train (列車)
//!   - TemplateSegment (テンプレート列車への部分参照)

use std::collections::hash_map::Entry;

use serde::{Deserialize, Serialize};

use crate::{
    command::CommandError, model::{
        DiagramRoot, ExtensionProperty, station::StationId, template_train::TemplateTrainId,
        time::Time, timetable::TimetableId,
    }, weaverail_id
};

weaverail_id!(TrainId, "TRA_");

/// Weaverail上の1つの「列車」を表す
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Train {
    /// 識別ID
    pub id: TrainId,
    /// 時刻表ID
    pub timetable_id: TimetableId,
    /// テンプレート列車ID
    pub template_segments: Vec<TemplateSegment>,
    /// 開始駅の出発時刻
    pub start_departure_time: Time,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl Train {
    pub fn new(id: TrainId, timetable_id: TimetableId) -> Self {
        Self {
            id,
            timetable_id,
            ..Default::default()
        }
    }
}
impl DiagramRoot {
    /// 列車を追加する関数
    /// 既に同一IDの列車が存在している場合はエラーを返す
    pub fn add_train(
        &mut self,
        train: Train,
    ) -> Result<(), CommandError> {
        match self.trains.entry(train.id) {
            Entry::Vacant(entry) => {
                entry.insert(train);
                Ok(())
            }
            Entry::Occupied(_) => Err(CommandError::DuplicateKey),
        }
    }

    /// 列車を削除する関数
    /// 指定IDの番線が存在しない場合はエラーを返す
    pub fn delete_train(
        &mut self,
        train_id: TrainId,
    ) -> Result<Train, CommandError> {
        self
            .trains
            .remove(&train_id)
            .ok_or(CommandError::TargetObjectNotFound)
    }

    pub fn get_stations(&self, train: &Train) -> Vec<StationId> {
        let mut result = Vec::new();
        for segment in &train.template_segments {
            let template_train = self
                .template_trains
                .get(&segment.template_train_id)
                .unwrap();
            let stations = template_train
                .get_filtered_stations(segment.start_station_id, segment.end_station_id);
            if result.is_empty() {
                result.extend(stations.iter().map(|sta| sta.station_id));
            } else {
                result.extend(stations[1..].iter().map(|sta| sta.station_id));
            }
        }

        result
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
