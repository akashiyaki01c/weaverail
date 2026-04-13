//! Weaverail上の「駅」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
//! - Station (駅)
//!   - Track (列車番線)

use crate::{
    command::CommandError,
    model::{DiagramRoot, ExtensionProperty},
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, hash_map::Entry};
use uuid::Uuid;

/// Weaverail上の駅に存在している1つの列車番線を表す構造体
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Track {
    /// 識別ID
    pub id: Uuid,
    /// 番線名 (例: "1番線")
    pub name: String,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl Track {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            ..Default::default()
        }
    }
}

/// Weaverail上の1つの駅を表す構造体
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Station {
    /// 識別ID
    pub id: Uuid,
    /// 正式駅名 (例: "梅田")
    pub name: String,
    /// 列車番線一覧
    pub tracks: HashMap<Uuid, Track>,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl Station {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            ..Default::default()
        }
    }
}

impl DiagramRoot {
    /// 駅を追加する関数
    /// 既に同一IDの駅が存在している場合はエラーを返す
    pub fn add_station(&mut self, station: Station) -> Result<(), CommandError> {
        match self.stations.entry(station.id) {
            Entry::Vacant(entry) => {
                entry.insert(station);
                Ok(())
            }
            Entry::Occupied(_) => Err(CommandError::DuplicateKey),
        }
    }

    /// 駅を削除する関数
    /// 指定IDの駅が存在しない場合はエラーを返す
    /// 路線から参照されている場合はエラーを返す
    pub fn delete_station(&mut self, station_id: Uuid) -> Result<Station, CommandError> {
        if self
            .lines
            .values()
            .any(|line| line.contains_station(station_id))
        {
            return Err(CommandError::ExternalReference);
        }
        self.stations
            .remove(&station_id)
            .ok_or(CommandError::TargetObjectNotFound)
    }

    /// 駅名から駅を検索する関数
    /// 見つからない場合は None を返す
    pub fn find_station_by_name(&self, station_name: &str) -> Option<&Station> {
        self.stations
            .values()
            .find(|station| station.name == station_name)
    }
}
