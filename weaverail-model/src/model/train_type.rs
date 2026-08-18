//! Weaverail上の「列車種別」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
//! - TrainType (列車種別)

use indexmap::map::Entry;

use serde::{Deserialize, Serialize};

use crate::path::Heddle;
use crate::{
    error::ModelError,
    model::{DiagramRoot, ExtensionProperty, PropertiableObject},
    weaverail_id,
};

weaverail_id!(TrainTypeId, "TYP_");

/// Weaverail上の1つの「列車種別」を表す構造体
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
pub struct TrainType {
    /// 識別ID
    pub id: TrainTypeId,
    /// 列車種別名 (例: "普通列車")
    pub name: String,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl TrainType {
    pub fn new(id: TrainTypeId, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            ..Default::default()
        }
    }
}

impl DiagramRoot {
    /// 列車種別を追加する関数
    /// 既に同一IDの列車種別が存在している場合はエラーを返す
    pub fn add_train_type(&mut self, train_type: TrainType) -> Result<(), ModelError> {
        match self.train_types.entry(train_type.id) {
            Entry::Vacant(entry) => {
                entry.insert(train_type);
                Ok(())
            }
            Entry::Occupied(_) => Err(ModelError::DuplicateKey),
        }
    }

    /// 列車種別を削除する関数
    /// 計算オーダは `O(template_trains.len)`
    /// 指定IDの列車種別が存在しない場合はエラーを返す
    /// テンプレート列車から讃匠されている場合はエラーを返す
    pub fn delete_train_type(
        &mut self,
        train_type_id: TrainTypeId,
    ) -> Result<TrainType, ModelError> {
        if self
            .template_trains
            .values()
            .any(|train| train.train_type_id == train_type_id)
        {
            return Err(ModelError::ExternalReferenced);
        }

        self.train_types
            .shift_remove(&train_type_id)
            .ok_or(ModelError::ObjectNotFound)
    }

    /// 列車種別名から列車種別を検索する関数
    /// 見つからない場合は None を返す
    pub fn find_train_type_by_name(&self, train_type_name: &str) -> Option<&TrainType> {
        self.train_types
            .values()
            .find(|train_type| train_type.name == train_type_name)
    }

    /// 列車種別データが正常な値であるかを検証する
    pub fn validate_train_type(&self, train_type_id: TrainTypeId) -> Result<(), ModelError> {
        let _ = self
            .train_types
            .get(&train_type_id)
            .ok_or(ModelError::ObjectNotFound)?;
        Ok(())
    }
}
impl PropertiableObject for TrainType {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        id::WeaverailId,
        station::Station,
        station::StationId,
        template_train::{StopType, TemplateTrain, TemplateTrainStation, TemplateTrainStationId, TemplateTrainId},
        time::Time,
        track::{Track, TrackId},
    };

    #[test]
    fn test_train_type_creation() {
        let train_type_id = TrainTypeId::new(WeaverailId::new(1));
        let train_type = TrainType::new(train_type_id, "普通");

        assert_eq!(train_type.id, train_type_id);
        assert_eq!(train_type.name, "普通");
        assert_eq!(train_type.properties, ExtensionProperty::new());
    }

    #[test]
    fn test_add_and_delete_train_type() {
        let mut root = DiagramRoot::default();
        let train_type_id = TrainTypeId::new(WeaverailId::new(1));
        let train_type = TrainType::new(train_type_id, "普通");

        assert!(root.add_train_type(train_type.clone()).is_ok());
        assert_eq!(root.train_types.len(), 1);

        let removed = root.delete_train_type(train_type_id);
        assert!(removed.is_ok());
        assert_eq!(removed.unwrap().name, "普通");
    }

    #[test]
    fn test_duplicate_train_type_id_error() {
        let mut root = DiagramRoot::default();
        let train_type_id = TrainTypeId::new(WeaverailId::new(1));

        root.add_train_type(TrainType::new(train_type_id, "普通")).unwrap();
        let result = root.add_train_type(TrainType::new(train_type_id, "快速"));
        assert_eq!(result.unwrap_err(), ModelError::DuplicateKey);
    }

    #[test]
    fn test_delete_nonexistent_train_type() {
        let mut root = DiagramRoot::default();
        let result = root.delete_train_type(TrainTypeId::new(WeaverailId::new(1)));
        assert_eq!(result.unwrap_err(), ModelError::ObjectNotFound);
    }

    #[test]
    fn test_find_train_type_by_name() {
        let mut root = DiagramRoot::default();
        root.add_train_type(TrainType::new(TrainTypeId::new(WeaverailId::new(1)), "普通")).unwrap();
        root.add_train_type(TrainType::new(TrainTypeId::new(WeaverailId::new(2)), "快速")).unwrap();

        assert_eq!(root.find_train_type_by_name("普通").unwrap().name, "普通");
        assert!(root.find_train_type_by_name("特急").is_none());
    }

    #[test]
    fn test_validate_train_type() {
        let mut root = DiagramRoot::default();
        let train_type_id = TrainTypeId::new(WeaverailId::new(1));
        root.add_train_type(TrainType::new(train_type_id, "普通")).unwrap();

        assert!(root.validate_train_type(train_type_id).is_ok());
        assert!(root.validate_train_type(TrainTypeId::new(WeaverailId::new(99))).is_err());
    }

    #[test]
    fn test_train_type_properties() {
        let mut train_type = TrainType::new(TrainTypeId::new(WeaverailId::new(1)), "普通");
        let value = Heddle::String("local".to_string());

        assert!(train_type.set_property("service_level", value.clone()).is_none());
        assert_eq!(train_type.get_property("service_level").unwrap(), &value);
        assert!(train_type.remove_property("service_level").is_some());
        assert!(train_type.get_property("service_level").is_none());
    }

    #[test]
    fn test_delete_train_type_referenced_by_template_train_is_rejected() {
        let mut root = DiagramRoot::default();
        let station_id = StationId::new(WeaverailId::new(1));
        let track_id = TrackId::new(WeaverailId::new(2));
        let train_type_id = TrainTypeId::new(WeaverailId::new(3));
        root.add_station(Station::new(station_id, "梅田")).unwrap();
        root.add_track(Track::new(track_id, station_id, "1番線")).unwrap();
        root.add_train_type(TrainType::new(train_type_id, "普通")).unwrap();

        let template_train = TemplateTrain {
            id: TemplateTrainId::new(WeaverailId::new(4)),
            name: "普通列車".to_string(),
            train_type_id,
            start_station: TemplateTrainStation {
                id: TemplateTrainStationId::new(WeaverailId::new(5)),
                station_id,
                track_id,
                stop_time: StopType::Stop(Time::new(0, 0, 30)),
                properties: ExtensionProperty::new(),
            },
            segments: Vec::new(),
            properties: ExtensionProperty::new(),
        };
        root.add_template_train(template_train).unwrap();

        let result = root.delete_train_type(train_type_id);
        assert_eq!(result.unwrap_err(), ModelError::ExternalReferenced);
    }

    #[test]
    fn test_multiple_train_types() {
        let mut root = DiagramRoot::default();
        let mut ids = Vec::new();
        for i in 1..=3 {
            let train_type_id = TrainTypeId::new(WeaverailId::new(i));
            ids.push(train_type_id);
            assert!(root.add_train_type(TrainType::new(train_type_id, &format!("種別{}", i))).is_ok());
        }

        assert_eq!(root.train_types.len(), 3);
        for train_type_id in ids {
            assert!(root.delete_train_type(train_type_id).is_ok());
        }
        assert_eq!(root.train_types.len(), 0);
    }
}
