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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        id::WeaverailId,
        station::{Station, StationId},
        template_train::{
            StopType, TemplateTrain, TemplateTrainId, TemplateTrainStation,
            TemplateTrainStationId,
        },
        time::Time,
        train_type::{TrainType, TrainTypeId},
    };

    #[test]
    fn test_track_creation() {
        let station_id = StationId::new(WeaverailId::new(1));
        let track_id = TrackId::new(WeaverailId::new(2));

        let track = Track::new(track_id, station_id, "1番線");

        assert_eq!(track.id, track_id);
        assert_eq!(track.station_id, station_id);
        assert_eq!(track.name, "1番線");
        assert_eq!(track.properties, ExtensionProperty::new());
    }

    #[test]
    fn test_add_and_delete_track() {
        let mut root = DiagramRoot::default();
        let station_id = StationId::new(WeaverailId::new(1));
        let track_id = TrackId::new(WeaverailId::new(2));

        root.add_station(Station::new(station_id, "梅田")).unwrap();
        let track = Track::new(track_id, station_id, "1番線");

        assert!(root.add_track(track.clone()).is_ok());
        assert_eq!(root.tracks.len(), 1);

        let removed = root.delete_track(track_id);
        assert!(removed.is_ok());
        assert_eq!(removed.unwrap().id, track_id);
        assert_eq!(root.tracks.len(), 0);
    }

    #[test]
    fn test_duplicate_track_id_error() {
        let mut root = DiagramRoot::default();
        let station_id = StationId::new(WeaverailId::new(1));
        let track_id = TrackId::new(WeaverailId::new(2));

        root.add_station(Station::new(station_id, "梅田")).unwrap();
        root.add_track(Track::new(track_id, station_id, "1番線")).unwrap();

        let result = root.add_track(Track::new(track_id, station_id, "2番線"));
        assert_eq!(result.unwrap_err(), ModelError::DuplicateKey);
    }

    #[test]
    fn test_delete_nonexistent_track() {
        let mut root = DiagramRoot::default();
        let track_id = TrackId::new(WeaverailId::new(2));

        let result = root.delete_track(track_id);
        assert_eq!(result.unwrap_err(), ModelError::ObjectNotFound);
    }

    #[test]
    fn test_track_station_lookup() {
        let station_id = StationId::new(WeaverailId::new(1));
        let track_id = TrackId::new(WeaverailId::new(2));
        let track = Track::new(track_id, station_id, "1番線");
        let mut root = DiagramRoot::default();
        root.add_station(Station::new(station_id, "梅田")).unwrap();

        let station = track.station(&root);
        assert!(station.is_ok());
        assert_eq!(station.unwrap().name, "梅田");
    }

    #[test]
    fn test_validate_track() {
        let mut root = DiagramRoot::default();
        let station_id = StationId::new(WeaverailId::new(1));
        let track_id = TrackId::new(WeaverailId::new(2));

        root.add_station(Station::new(station_id, "梅田")).unwrap();
        root.add_track(Track::new(track_id, station_id, "1番線")).unwrap();

        assert!(root.validate_track(track_id).is_ok());
        assert!(root.validate_track(TrackId::new(WeaverailId::new(99))).is_err());
    }

    #[test]
    fn test_track_properties() {
        let mut track = Track::new(TrackId::new(WeaverailId::new(2)), StationId::new(WeaverailId::new(1)), "1番線");
        let value = Heddle::String("platform".to_string());

        assert!(track.set_property("platform_type", value.clone()).is_none());
        assert_eq!(track.get_property("platform_type").unwrap(), &value);
        assert!(track.remove_property("platform_type").is_some());
        assert!(track.get_property("platform_type").is_none());
    }

    #[test]
    fn test_delete_track_referenced_by_template_train_is_rejected() {
        let mut root = DiagramRoot::default();
        let station_id = StationId::new(WeaverailId::new(1));
        let track_id = TrackId::new(WeaverailId::new(2));
        let train_type_id = TrainTypeId::new(WeaverailId::new(3));
        let template_train_id = TemplateTrainId::new(WeaverailId::new(4));

        root.add_station(Station::new(station_id, "梅田")).unwrap();
        root.add_track(Track::new(track_id, station_id, "1番線")).unwrap();
        root.add_train_type(TrainType::new(train_type_id, "普通")).unwrap();

        let template_train = TemplateTrain {
            id: template_train_id,
            name: "普通列車".to_string(),
            train_type_id: train_type_id,
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

        let result = root.delete_track(track_id);
        assert_eq!(result.unwrap_err(), ModelError::ExternalReferenced);
    }

    #[test]
    fn test_multiple_tracks() {
        let mut root = DiagramRoot::default();
        let station_id = StationId::new(WeaverailId::new(1));
        root.add_station(Station::new(station_id, "梅田")).unwrap();

        let mut ids = Vec::new();
        for i in 1..=3 {
            let track_id = TrackId::new(WeaverailId::new(i + 10));
            ids.push(track_id);
            assert!(root.add_track(Track::new(track_id, station_id, &format!("{}番線", i))).is_ok());
        }

        assert_eq!(root.tracks.len(), 3);
        for track_id in ids {
            assert!(root.delete_track(track_id).is_ok());
        }
        assert_eq!(root.tracks.len(), 0);
    }
}
