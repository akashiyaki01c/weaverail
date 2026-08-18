use weaverail_model::{
    error::ModelError,
    model::{DiagramRoot, Station, Track, TrackId},
};

/// 1 つの番線を `DiagramRoot` に追加する。
///
/// 番線 ID の重複は `ModelError::DuplicateKey` で扱う。
pub fn add_track(root: &mut DiagramRoot, track: Track) -> Result<(), ModelError> {
    root.add_track(track)
}

/// 指定した番線 ID を削除し、削除前の番線を返す。
pub fn delete_track(root: &mut DiagramRoot, track_id: TrackId) -> Result<Track, ModelError> {
    root.delete_track(track_id)
}

/// 番線が参照している駅・ID 情報が整合しているか検証する。
pub fn validate_track(root: &DiagramRoot, track_id: TrackId) -> Result<(), ModelError> {
    root.validate_track(track_id)
}

/// 番線に紐づく駅を取得する。
///
/// 参照先が存在しない場合は `ModelError::ObjectNotFound` を返す。
pub fn station_of_track<'a>(root: &'a DiagramRoot, track: &Track) -> Result<&'a Station, ModelError> {
    track.station(root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use weaverail_model::model::{
        id::WeaverailId,
        station::StationId,
    };

    #[test]
    fn operation_add_and_delete_track_matches_model_behavior() {
        let mut root = DiagramRoot::default();
        let station_id = StationId::new(WeaverailId::new(1));
        root.add_station(Station::new(station_id, "梅田")).unwrap();

        let track_id = TrackId::new(WeaverailId::new(2));
        let track = Track::new(track_id, station_id, "1番線");

        assert!(add_track(&mut root, track.clone()).is_ok());
        assert_eq!(root.tracks.len(), 1);

        let removed = delete_track(&mut root, track_id);
        assert!(removed.is_ok());
        assert_eq!(removed.unwrap().id, track_id);
    }

    #[test]
    fn operation_validate_track_matches_model_behavior() {
        let mut root = DiagramRoot::default();
        let station_id = StationId::new(WeaverailId::new(1));
        root.add_station(Station::new(station_id, "梅田")).unwrap();

        let track_id = TrackId::new(WeaverailId::new(2));
        let track = Track::new(track_id, station_id, "1番線");
        add_track(&mut root, track).unwrap();

        assert!(validate_track(&root, track_id).is_ok());
        assert!(validate_track(&root, TrackId::new(WeaverailId::new(99))).is_err());
    }

    #[test]
    fn operation_station_of_track_matches_model_behavior() {
        let mut root = DiagramRoot::default();
        let station_id = StationId::new(WeaverailId::new(1));
        root.add_station(Station::new(station_id, "梅田")).unwrap();

        let track = Track::new(TrackId::new(WeaverailId::new(2)), station_id, "1番線");
        let station = station_of_track(&root, &track);

        assert!(station.is_ok());
        assert_eq!(station.unwrap().name, "梅田");
    }
}
