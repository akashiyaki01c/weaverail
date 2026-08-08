//! Weaverail上の「駅」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
//! - Station (駅)
//!   - Track (列車番線)

use crate::path::Heddle;
use crate::{
    error::ModelError,
    model::{
        DiagramRoot, ExtensionProperty, PropertiableObject,
        station::{Station, StationId},
    },
    weaverail_id,
};
use indexmap::map::Entry;
use serde::{Deserialize, Serialize};

weaverail_id!(TrackId, "TRC_");

/// Weaverail上の駅に存在している1つの列車番線を表す構造体
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
pub struct Track {
    /// 識別ID
    pub id: TrackId,
    /// 駅ID
    pub station_id: StationId,
    /// 番線名 (例: "1番線")
    pub name: String,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl Track {
    pub fn new(id: TrackId, station_id: StationId, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            station_id,
            ..Default::default()
        }
    }

    /// 所属駅を取得する関数
    /// 計算量は `O(1)`
    pub fn station<'a>(&self, root: &'a DiagramRoot) -> Result<&'a Station, ModelError> {
        root.stations
            .get(&self.station_id)
            .ok_or(ModelError::ObjectNotFound)
    }
}
impl DiagramRoot {
    /// 列車番線を追加する関数
    /// 計算オーダは `O(1)`
    /// 既に同一IDの番線が存在している場合はエラーを返す
    pub fn add_track(&mut self, track: Track) -> Result<(), ModelError> {
        match self.tracks.entry(track.id) {
            Entry::Vacant(entry) => {
                entry.insert(track);
                Ok(())
            }
            Entry::Occupied(_) => Err(ModelError::DuplicateKey),
        }
    }

    /// 番線を削除する関数
    /// 計算オーダは `O(track.len)`
    /// 指定IDの番線が存在しない場合はエラーを返す
    /// テンプレート列車から参照されている場合はエラーを返す
    pub fn delete_track(&mut self, track_id: TrackId) -> Result<Track, ModelError> {
        if self
            .template_trains
            .values()
            .any(|train| train.contains_track(track_id))
        {
            return Err(ModelError::ExternalReferenced);
        }
        self.tracks
            .shift_remove(&track_id)
            .ok_or(ModelError::ObjectNotFound)
    }

    /// 番線データが正常な値であるかを検証する
    pub fn validate_track(&self, track_id: TrackId) -> Result<(), ModelError> {
        let track = self
            .tracks
            .get(&track_id)
            .ok_or(ModelError::ObjectNotFound)?;
        let _ = self
            .stations
            .get(&track.station_id)
            .ok_or(ModelError::ObjectNotFound)?;
        Ok(())
    }
}
impl PropertiableObject for Track {
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
