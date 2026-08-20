use weaverail_model::{
    error::ModelError,
    model::{DiagramRoot, LineSegment, LineSegmentId},
};

/// 駅間を新規追加する。
///
/// 既に同一 ID があれば `ModelError::DuplicateKey` を返す。
pub fn add_segment(root: &mut DiagramRoot, segment: LineSegment) -> Result<(), ModelError> {
    root.add_segment(segment)
}

/// 指定した駅間 ID を削除し、前の値を返す。
///
/// 路線やテンプレート列車から参照されている場合は `ModelError::ExternalReferenced` を返す。
pub fn delete_segment(
    root: &mut DiagramRoot,
    segment_id: LineSegmentId,
) -> Result<LineSegment, ModelError> {
    root.delete_segment(segment_id)
}

/// 駅間の参照先が整合しているか検証する。
pub fn validate_segment(root: &DiagramRoot, segment_id: LineSegmentId) -> Result<(), ModelError> {
    root.validate_segment(segment_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use weaverail_model::model::{id::WeaverailId, station::Station, station::StationId};

    #[test]
    fn operation_add_and_delete_segment_matches_model_behavior() {
        let mut root = DiagramRoot::default();
        let start = StationId::new(WeaverailId::new(10));
        let end = StationId::new(WeaverailId::new(11));
        root.add_station(Station::new(start, "梅田")).unwrap();
        root.add_station(Station::new(end, "大阪")).unwrap();

        let segment_id = LineSegmentId::new(WeaverailId::new(1));
        let segment = LineSegment::new(segment_id, start, end);

        assert!(add_segment(&mut root, segment.clone()).is_ok());
        assert_eq!(root.segments.len(), 1);

        let removed = delete_segment(&mut root, segment_id);
        assert!(removed.is_ok());
        assert_eq!(removed.unwrap().id, segment_id);
    }

    #[test]
    fn operation_validate_segment_matches_model_behavior() {
        let mut root = DiagramRoot::default();
        let start = StationId::new(WeaverailId::new(10));
        let end = StationId::new(WeaverailId::new(11));
        root.add_station(Station::new(start, "梅田")).unwrap();
        root.add_station(Station::new(end, "大阪")).unwrap();

        let segment_id = LineSegmentId::new(WeaverailId::new(1));
        add_segment(&mut root, LineSegment::new(segment_id, start, end)).unwrap();

        assert!(validate_segment(&root, segment_id).is_ok());
        assert!(validate_segment(&root, LineSegmentId::new(WeaverailId::new(99))).is_err());
    }
}
