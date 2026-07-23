//! Weaverail上の「駅」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
//! - Station (駅)
//!   - Track (列車番線)

use crate::{
    error::ModelError, model::{DiagramRoot, ExtensionProperty, PropertiableObject}, weaverail_id,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::hash_map::Entry;
use weaverail_object::WeaverailDNA;

weaverail_id!(StationId, "STA_");

/// Weaverail上の1つの駅を表す構造体
#[derive(WeaverailDNA, ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
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
            .remove(&station_id)
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
        let _ = self.stations.get(&station_id).ok_or(ModelError::ObjectNotFound)?;
        Ok(())
    }
}
impl PropertiableObject for Station {
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

#[test]
fn dna_test() {
    println!("{:?}", Station::print_dna_info());
}
