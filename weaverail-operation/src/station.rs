use weaverail_model::{
    error::ModelError,
    model::{DiagramRoot, Station, StationId},
};

/// `DiagramRoot` に新しい駅を追加する。
///
/// 既存の ID が重複している場合は `ModelError::DuplicateKey` を返す。
pub fn add_station(root: &mut DiagramRoot, station: Station) -> Result<(), ModelError> {
    root.add_station(station)
}

/// 指定した駅 ID の駅を削除し、削除前の値を返す。
///
/// 参照が残っている場合には `ModelError` で失敗することがある。
pub fn delete_station(
    root: &mut DiagramRoot,
    station_id: StationId,
) -> Result<Station, ModelError> {
    root.delete_station(station_id)
}

/// 駅名から駅を探索する。
///
/// 検索結果は `&Station` の参照として返し、見つからなければ `None` を返す。
pub fn find_station_by_name<'a>(root: &'a DiagramRoot, station_name: &str) -> Option<&'a Station> {
    root.find_station_by_name(station_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use weaverail_model::model::id::WeaverailId;

    #[test]
    fn operation_add_and_delete_station_matches_model_behavior() {
        let mut root = DiagramRoot::default();
        let station_id = StationId::new(WeaverailId::new(1));
        let station = Station::new(station_id, "梅田");

        assert!(add_station(&mut root, station.clone()).is_ok());
        assert_eq!(root.stations.len(), 1);

        let removed = delete_station(&mut root, station_id);
        assert!(removed.is_ok());
        assert_eq!(removed.unwrap().name, "梅田");
    }

    #[test]
    fn operation_find_station_by_name_matches_model_behavior() {
        let mut root = DiagramRoot::default();
        root.add_station(Station::new(StationId::new(WeaverailId::new(1)), "京都"))
            .unwrap();
        root.add_station(Station::new(StationId::new(WeaverailId::new(2)), "大阪"))
            .unwrap();

        assert_eq!(find_station_by_name(&root, "京都").unwrap().name, "京都");
        assert!(find_station_by_name(&root, "東京").is_none());
    }
}
