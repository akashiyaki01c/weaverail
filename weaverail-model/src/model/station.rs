//! Weaverail上の「駅」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
//! - Station (駅)
//!   - Track (列車番線)

use crate::path::Heddle;
use crate::{
    error::ModelError,
    model::{DiagramRoot, ExtensionProperty, PropertiableObject},
    weaverail_id,
};

use indexmap::map::Entry;
use serde::{Deserialize, Serialize};
weaverail_id!(StationId, "STA_");

/// Weaverail上の1つの駅を表す構造体
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
pub struct Station {
    /// 識別ID
    pub id: StationId,
    /// 正式駅名 (例: "梅田")
    pub name: String,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl Station {
    pub fn new(id: StationId, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            ..Default::default()
        }
    }
}

impl DiagramRoot {
    /// 駅を追加する関数
    /// 計算オーダは `O(1)`
    /// 既に同一IDの駅が存在している場合はエラーを返す
    pub fn add_station(&mut self, station: Station) -> Result<(), ModelError> {
        match self.stations.entry(station.id) {
            Entry::Vacant(entry) => {
                entry.insert(station);
                Ok(())
            }
            Entry::Occupied(_) => Err(ModelError::DuplicateKey),
        }
    }

    /// 駅を削除する関数
    /// 計算オーダは `O(segments.len + track.len)`
    /// 指定IDの駅が存在しない場合はエラーを返す
    /// 駅間から参照されている場合はエラーを返す
    /// 番線から参照されている場合はエラーを返す
    pub fn delete_station(&mut self, station_id: StationId) -> Result<Station, ModelError> {
        if self
            .segments
            .values()
            .any(|segment| segment.contains_station(station_id))
        {
            return Err(ModelError::ExternalReferenced);
        }
        if self
            .tracks
            .values()
            .any(|track| track.station_id == station_id)
        {
            return Err(ModelError::ExternalReferenced);
        }
        self.stations
            .shift_remove(&station_id)
            .ok_or(ModelError::ObjectNotFound)
    }

    /// 駅名から駅を検索する関数
    /// 見つからない場合は None を返す
    pub fn find_station_by_name(&self, station_name: &str) -> Option<&Station> {
        self.stations
            .values()
            .find(|station| station.name == station_name)
    }

    /// 駅データが正常な値であるかを検証する
    pub fn validate_station(&self, station_id: StationId) -> Result<(), ModelError> {
        let _ = self
            .stations
            .get(&station_id)
            .ok_or(ModelError::ObjectNotFound)?;
        Ok(())
    }
}
impl PropertiableObject for Station {
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
    use crate::model::id::WeaverailId;

    /// Station の生成と基本プロパティが正しく設定されることをテスト
    #[test]
    fn test_station_creation() {
        let station_id = StationId::new(WeaverailId::new(1));
        let station = Station::new(station_id, "梅田");

        assert_eq!(station.id, station_id);
        assert_eq!(station.name, "梅田");
        assert_eq!(station.properties, ExtensionProperty::new());
    }

    /// Station の名前が正しく設定・変更されることをテスト
    #[test]
    fn test_station_name_change() {
        let station_id = StationId::new(WeaverailId::new(1));
        let mut station = Station::new(station_id, "梅田");

        assert_eq!(station.name, "梅田");

        station.name = "北梅田".to_string();
        assert_eq!(station.name, "北梅田");
    }

    /// DiagramRoot に Station を追加・削除できることをテスト
    #[test]
    fn test_add_and_delete_station() {
        let mut root = DiagramRoot::default();
        let station_id = StationId::new(WeaverailId::new(1));
        let station = Station::new(station_id, "京都");

        // 追加テスト
        assert!(root.add_station(station.clone()).is_ok());
        assert_eq!(root.stations.len(), 1);
        assert_eq!(
            root.stations.get(&station_id).unwrap().name,
            "京都"
        );

        // 削除テスト
        let removed_station = root.delete_station(station_id);
        assert!(removed_station.is_ok());
        assert_eq!(removed_station.unwrap().name, "京都");
        assert_eq!(root.stations.len(), 0);
    }

    /// 同一IDの Station を2つ追加しようとするとエラーになることをテスト
    #[test]
    fn test_duplicate_station_id_error() {
        let mut root = DiagramRoot::default();
        let station_id = StationId::new(WeaverailId::new(1));
        let station1 = Station::new(station_id, "梅田");
        let station2 = Station::new(station_id, "北梅田");

        assert!(root.add_station(station1).is_ok());

        // 同一IDで追加しようとする
        let result = root.add_station(station2);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ModelError::DuplicateKey);
        assert_eq!(root.stations.len(), 1);
    }

    /// 存在しない Station ID を削除しようとするとエラーになることをテスト
    #[test]
    fn test_delete_nonexistent_station() {
        let mut root = DiagramRoot::default();
        let station_id = StationId::new(WeaverailId::new(1));

        let result = root.delete_station(station_id);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ModelError::ObjectNotFound);
    }

    /// Station を名前で検索できることをテスト
    #[test]
    fn test_find_station_by_name() {
        let mut root = DiagramRoot::default();
        let station1 = Station::new(StationId::new(WeaverailId::new(1)), "京都");
        let station2 = Station::new(StationId::new(WeaverailId::new(2)), "大阪");

        root.add_station(station1.clone()).unwrap();
        root.add_station(station2.clone()).unwrap();

        // 存在する駅を検索
        let found = root.find_station_by_name("京都");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "京都");

        // 存在しない駅を検索
        let not_found = root.find_station_by_name("東京");
        assert!(not_found.is_none());
    }

    /// Station の拡張プロパティを取得・設定・削除できることをテスト
    #[test]
    fn test_station_properties() {
        let station_id = StationId::new(WeaverailId::new(1));
        let mut station = Station::new(station_id, "梅田");

        // プロパティを設定
        let value = Heddle::String("test_value".to_string());
        let result = station.set_property("custom_prop", value.clone());
        assert!(result.is_none());

        // プロパティを取得
        let retrieved = station.get_property("custom_prop");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), &value);

        // プロパティを上書き
        let new_value = Heddle::String("new_value".to_string());
        let overwritten = station.set_property("custom_prop", new_value.clone());
        assert!(overwritten.is_some());

        // プロパティを削除
        let removed = station.remove_property("custom_prop");
        assert!(removed.is_some());
        assert!(station.get_property("custom_prop").is_none());
    }

    /// Station を validate できることをテスト
    #[test]
    fn test_validate_station() {
        let mut root = DiagramRoot::default();
        let station_id = StationId::new(WeaverailId::new(1));
        let station = Station::new(station_id, "大阪");

        // 存在しない駅を validate
        let result = root.validate_station(station_id);
        assert!(result.is_err());

        // 駅を追加して validate
        root.add_station(station).unwrap();
        let result = root.validate_station(station_id);
        assert!(result.is_ok());
    }

    /// 複数の Station を追加・管理できることをテスト
    #[test]
    fn test_multiple_stations() {
        let mut root = DiagramRoot::default();
        let mut station_ids = Vec::new();

        // 5つの駅を追加
        for i in 1..=5 {
            let station_id = StationId::new(WeaverailId::new(i));
            let station = Station::new(station_id, &format!("駅{}", i));
            station_ids.push(station_id);
            assert!(root.add_station(station).is_ok());
        }

        assert_eq!(root.stations.len(), 5);

        // 全駅を削除
        for station_id in station_ids {
            assert!(root.delete_station(station_id).is_ok());
        }

        assert_eq!(root.stations.len(), 0);
    }
}
