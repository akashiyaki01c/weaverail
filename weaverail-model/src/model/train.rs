//! Weaverail上の「列車」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
//! - Train (列車)
//!   - TemplateSegment (テンプレート列車への部分参照)

use indexmap::map::Entry;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    error::ModelError,
    model::{
        DiagramRoot, ExtensionProperty, PropertiableObject,
        station::{Station, StationId},
        template_train::{TemplateTrain, TemplateTrainId, TemplateTrainSegment},
        time::Time,
        timetable::{Timetable, TimetableId},
    },
    weaverail_id,
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

    /// 時刻表を取得する関数
    /// 計算量は `O(1)`
    pub fn timetable<'a>(&self, root: &'a DiagramRoot) -> Result<&'a Timetable, ModelError> {
        root.timetables
            .get(&self.timetable_id)
            .ok_or(ModelError::ObjectNotFound)
    }

    /// 列車に指定のテンプレート列車が含まれているか
    pub fn contain_template_train(&self, template_train_id: TemplateTrainId) -> bool {
        self.template_segments
            .iter()
            .any(|seg| seg.template_train_id == template_train_id)
    }
}
impl DiagramRoot {
    /// 列車を追加する関数
    /// 計算オーダは `O(1)`
    /// 既に同一IDの列車が存在している場合はエラーを返す
    pub fn add_train(&mut self, train: Train) -> Result<(), ModelError> {
        match self.trains.entry(train.id) {
            Entry::Vacant(entry) => {
                entry.insert(train);
                Ok(())
            }
            Entry::Occupied(_) => Err(ModelError::DuplicateKey),
        }
    }

    /// 列車を削除する関数
    /// 計算オーダは `O(1)`
    /// 指定IDの列車が存在しない場合はエラーを返す
    pub fn delete_train(&mut self, train_id: TrainId) -> Result<Train, ModelError> {
        for timetable in self.timetables.values_mut() {
            for order in timetable.segment_train_orders.values_mut() {
                if let Some(index) = order.0.order.iter().position(|v| *v == train_id) {
                    order.0.order.remove(index);
                }
                if let Some(index) = order.1.order.iter().position(|v| *v == train_id) {
                    order.0.order.remove(index);
                }
            }
        }
        self.trains
            .shift_remove(&train_id)
            .ok_or(ModelError::ObjectNotFound)
    }

    /// 列車の経由する駅を列挙する関数
    pub fn get_train_stations(&self, train: &Train) -> Result<Vec<StationId>, ModelError> {
        let mut result = Vec::new();
        for segment in &train.template_segments {
            let template_train = self
                .template_trains
                .get(&segment.template_train_id)
                .ok_or(ModelError::ObjectNotFound)?;
            let stations: Vec<&super::template_train::TemplateTrainStation> = template_train
                .get_filtered_stations(segment.start_station_id, segment.end_station_id)?;
            if result.is_empty() {
                result.extend(stations.iter().map(|sta| sta.station_id));
            } else {
                result.extend(stations[1..].iter().map(|sta| sta.station_id));
            }
        }

        Ok(result)
    }

    /// 列車の経由する駅間を列挙する関数
    pub fn get_train_segment(
        &self,
        train: &Train,
    ) -> Result<Vec<TemplateTrainSegment>, ModelError> {
        let mut result = Vec::new();
        for segment in &train.template_segments {
            let template_train = self
                .template_trains
                .get(&segment.template_train_id)
                .ok_or(ModelError::ObjectNotFound)?;
            let segments = template_train
                .get_filtered_segment(segment.start_station_id, segment.end_station_id)?;
            result.extend(segments.1.iter().map(|v| v.0.clone()));
        }
        Ok(result)
    }

    /// 列車種別データが正常な値であるかを検証する
    pub fn validate_train(&self, train_id: TrainId) -> Result<(), ModelError> {
        let train = self
            .trains
            .get(&train_id)
            .ok_or(ModelError::ObjectNotFound)?;
        for seg in train.template_segments.iter() {
            let _ = self
                .template_trains
                .get(&seg.template_train_id)
                .ok_or(ModelError::ObjectNotFound)?;
        }
        Ok(())
    }
}
impl PropertiableObject for Train {
    fn get_property(&self, id: &str) -> Option<&Value> {
        self.properties.get(id)
    }

    fn set_property(&mut self, id: &str, value: Value) -> Option<Value> {
        self.properties.set(id, value)
    }

    fn remove_property(&mut self, id: &str) -> Option<Value> {
        self.properties.remove(id)
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
impl TemplateSegment {
    /// テンプレート列車を取得する関数
    /// 計算量は `O(1)`
    pub fn template_train<'a>(
        &self,
        root: &'a DiagramRoot,
    ) -> Result<&'a TemplateTrain, ModelError> {
        root.template_trains
            .get(&self.template_train_id)
            .ok_or(ModelError::ObjectNotFound)
    }
    /// 開始駅を取得する関数
    /// 計算量は `O(1)`
    pub fn start_station<'a>(&self, root: &'a DiagramRoot) -> Result<&'a Station, ModelError> {
        root.stations
            .get(&self.start_station_id)
            .ok_or(ModelError::ObjectNotFound)
    }
    /// 終了駅を取得する関数
    /// 計算量は `O(1)`
    pub fn end_station<'a>(&self, root: &'a DiagramRoot) -> Result<&'a Station, ModelError> {
        root.stations
            .get(&self.end_station_id)
            .ok_or(ModelError::ObjectNotFound)
    }
}
