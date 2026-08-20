//! Weaverail上の「列車」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
//! - Train (列車)
//!   - TemplateSegment (テンプレート列車への部分参照)

use indexmap::map::Entry;

use serde::{Deserialize, Serialize};

use crate::path::Heddle;
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
#[derive(
    weaverail_object::RnaObjectable,
    ts_rs::TS,
    Clone,
    PartialEq,
    Debug,
    Default,
    Serialize,
    Deserialize,
)]
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
                if let Some(index) = order.prograde.order.iter().position(|v| *v == train_id) {
                    order.prograde.order.remove(index);
                }
                if let Some(index) = order.retrograde.order.iter().position(|v| *v == train_id) {
                    order.retrograde.order.remove(index);
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
            result.extend(segments.1.iter().map(|section| section.segment.clone()));
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
    fn get_property(&self, id: &str) -> Option<&Heddle> {
        self.properties.get(id)
    }

    fn set_property(&mut self, id: &str, value: Heddle) -> Option<Heddle> {
        self.properties.set(id, value)
    }

    fn remove_property(&mut self, id: &str) -> Option<Heddle> {
        self.properties.remove(id)
    }
}

/// Weaverail上のテンプレート列車への部分参照
#[derive(
    weaverail_object::RnaObjectable,
    ts_rs::TS,
    Clone,
    PartialEq,
    Debug,
    Default,
    Serialize,
    Deserialize,
)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::id::WeaverailId;

    /// Train の生成と基本プロパティが正しく設定されることをテスト
    #[test]
    fn test_train_creation() {
        let train_id = TrainId::new(WeaverailId::new(1));
        let timetable_id = TimetableId::new(WeaverailId::new(1));
        let train = Train::new(train_id, timetable_id);

        assert_eq!(train.id, train_id);
        assert_eq!(train.timetable_id, timetable_id);
        assert_eq!(train.template_segments.len(), 0);
        assert_eq!(train.start_departure_time, Time::new_from_total_second(0));
        assert_eq!(train.properties, ExtensionProperty::new());
    }

    /// DiagramRoot に Train を追加・削除できることをテスト
    #[test]
    fn test_add_and_delete_train() {
        let mut root = DiagramRoot::default();
        let train_id = TrainId::new(WeaverailId::new(1));
        let timetable_id = TimetableId::new(WeaverailId::new(1));
        let train = Train::new(train_id, timetable_id);

        // 追加テスト
        assert!(root.add_train(train.clone()).is_ok());
        assert_eq!(root.trains.len(), 1);
        assert_eq!(
            root.trains.get(&train_id).unwrap().timetable_id,
            timetable_id
        );

        // 削除テスト
        let removed_train = root.delete_train(train_id);
        assert!(removed_train.is_ok());
        assert_eq!(removed_train.unwrap().id, train_id);
        assert_eq!(root.trains.len(), 0);
    }

    /// 同一IDの Train を2つ追加しようとするとエラーになることをテスト
    #[test]
    fn test_duplicate_train_id_error() {
        let mut root = DiagramRoot::default();
        let train_id = TrainId::new(WeaverailId::new(1));
        let timetable_id = TimetableId::new(WeaverailId::new(1));
        let train1 = Train::new(train_id, timetable_id);
        let train2 = Train::new(train_id, timetable_id);

        assert!(root.add_train(train1).is_ok());

        // 同一IDで追加しようとする
        let result = root.add_train(train2);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ModelError::DuplicateKey);
        assert_eq!(root.trains.len(), 1);
    }

    /// 存在しない Train ID を削除しようとするとエラーになることをテスト
    #[test]
    fn test_delete_nonexistent_train() {
        let mut root = DiagramRoot::default();
        let train_id = TrainId::new(WeaverailId::new(1));

        let result = root.delete_train(train_id);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ModelError::ObjectNotFound);
    }

    /// Train の出発時刻が正しく設定・変更されることをテスト
    #[test]
    fn test_train_departure_time() {
        let train_id = TrainId::new(WeaverailId::new(1));
        let timetable_id = TimetableId::new(WeaverailId::new(1));
        let mut train = Train::new(train_id, timetable_id);

        // デフォルトは 00:00:00
        assert_eq!(train.start_departure_time, Time::new_from_total_second(0));

        // 時刻を設定
        train.start_departure_time = Time::new_from_total_second(3600); // 01:00:00
        assert_eq!(
            train.start_departure_time,
            Time::new_from_total_second(3600)
        );
    }

    /// Train の拡張プロパティを取得・設定・削除できることをテスト
    #[test]
    fn test_train_properties() {
        let train_id = TrainId::new(WeaverailId::new(1));
        let timetable_id = TimetableId::new(WeaverailId::new(1));
        let mut train = Train::new(train_id, timetable_id);

        // プロパティを設定
        let value = Heddle::String("express".to_string());
        let result = train.set_property("train_type", value.clone());
        assert!(result.is_none());

        // プロパティを取得
        let retrieved = train.get_property("train_type");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), &value);

        // プロパティを削除
        let removed = train.remove_property("train_type");
        assert!(removed.is_some());
        assert!(train.get_property("train_type").is_none());
    }

    /// Train に template_segments が存在しない場合、contain_template_train は false を返す
    #[test]
    fn test_contain_template_train_empty() {
        let train_id = TrainId::new(WeaverailId::new(1));
        let timetable_id = TimetableId::new(WeaverailId::new(1));
        let train = Train::new(train_id, timetable_id);
        let template_train_id = TemplateTrainId::new(WeaverailId::new(1));

        assert!(!train.contain_template_train(template_train_id));
    }

    /// Train に template_segments が存在する場合、contain_template_train は true を返す
    #[test]
    fn test_contain_template_train_exists() {
        let train_id = TrainId::new(WeaverailId::new(1));
        let timetable_id = TimetableId::new(WeaverailId::new(1));
        let mut train = Train::new(train_id, timetable_id);

        let template_train_id = TemplateTrainId::new(WeaverailId::new(1));
        let start_station_id = StationId::new(WeaverailId::new(1));
        let end_station_id = StationId::new(WeaverailId::new(2));

        let template_segment = TemplateSegment {
            template_train_id,
            start_station_id,
            end_station_id,
        };

        train.template_segments.push(template_segment);

        assert!(train.contain_template_train(template_train_id));
    }

    /// 複数の Train を追加・管理できることをテスト
    #[test]
    fn test_multiple_trains() {
        let mut root = DiagramRoot::default();
        let timetable_id = TimetableId::new(WeaverailId::new(1));
        let mut train_ids = Vec::new();

        // 5つの列車を追加
        for i in 1..=5 {
            let train_id = TrainId::new(WeaverailId::new(i));
            let train = Train::new(train_id, timetable_id);
            train_ids.push(train_id);
            assert!(root.add_train(train).is_ok());
        }

        assert_eq!(root.trains.len(), 5);

        // 全列車を削除
        for train_id in train_ids {
            assert!(root.delete_train(train_id).is_ok());
        }

        assert_eq!(root.trains.len(), 0);
    }

    /// TemplateSegment の生成と基本プロパティが正しく設定されることをテスト
    #[test]
    fn test_template_segment_creation() {
        let template_train_id = TemplateTrainId::new(WeaverailId::new(1));
        let start_station_id = StationId::new(WeaverailId::new(1));
        let end_station_id = StationId::new(WeaverailId::new(2));

        let segment = TemplateSegment {
            template_train_id,
            start_station_id,
            end_station_id,
        };

        assert_eq!(segment.template_train_id, template_train_id);
        assert_eq!(segment.start_station_id, start_station_id);
        assert_eq!(segment.end_station_id, end_station_id);
    }
}
