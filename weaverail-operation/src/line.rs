use weaverail_model::{
    error::ModelError,
    model::{DiagramRoot, Line, LineId, LineSegment, LineSegmentId, SegmentRef, Station},
};

/// 路線を `DiagramRoot` に追加する。
///
/// 同一 ID の既存路線があれば `ModelError::DuplicateKey` を返す。
pub fn add_line(root: &mut DiagramRoot, line: Line) -> Result<(), ModelError> {
    root.add_line(line)
}

/// 指定した路線 ID を削除し、削除前の路線を返す。
pub fn delete_line(root: &mut DiagramRoot, line_id: LineId) -> Result<Line, ModelError> {
    root.delete_line(line_id)
}

/// 路線を構成する駅の一覧を、順番付きで取得する。
pub fn get_line_stations<'a>(
    root: &'a DiagramRoot,
    line: &Line,
) -> Result<Vec<&'a Station>, ModelError> {
    root.get_line_stations(line)
}

/// 駅間 ID から駅間本体を取得する。
pub fn get_segment(
    root: &DiagramRoot,
    segment_id: LineSegmentId,
) -> Option<&LineSegment> {
    root.get_segment(segment_id)
}

/// 路線の末尾に駅間を追加する。
///
/// 追加先の終端と接続されていない駅間を渡すと `ModelError::Error` を返す。
pub fn push_back_line_segment(
    root: &mut DiagramRoot,
    line_id: LineId,
    segment_id: LineSegmentId,
    is_reversed: bool,
) -> Result<(), ModelError> {
    root.push_back_line_segment(line_id, segment_id, is_reversed)
}

/// 路線の先頭に駅間を追加する。
pub fn push_front_line_segment(
    root: &mut DiagramRoot,
    line_id: LineId,
    segment_id: LineSegmentId,
    is_reversed: bool,
) -> Result<(), ModelError> {
    root.push_front_line_segment(line_id, segment_id, is_reversed)
}

/// 路線の末尾の駅間参照を取り出す。
pub fn pop_back_line_segment(
    root: &mut DiagramRoot,
    line_id: LineId,
) -> Result<SegmentRef, ModelError> {
    root.pop_back_line_segment(line_id)
}

/// 路線の先頭の駅間参照を取り出す。
pub fn pop_front_line_segment(
    root: &mut DiagramRoot,
    line_id: LineId,
) -> Result<SegmentRef, ModelError> {
    root.pop_front_line_segment(line_id)
}

/// 始発駅名と終着駅名から該当する駅間参照を検索する。
pub fn find_segment_by_name<'a>(
    root: &'a DiagramRoot,
    start_station_name: &str,
    end_station_name: &str,
) -> Result<&'a SegmentRef, ModelError> {
    root.find_segment_by_name(start_station_name, end_station_name)
}

/// 路線の整合性を検証する。
///
/// 参照されている駅間が存在しない場合は `ModelError::ObjectNotFound` を返す。
pub fn validate_line(root: &DiagramRoot, line_id: LineId) -> Result<(), ModelError> {
    root.validate_line(line_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use weaverail_model::model::{
        id::WeaverailId,
        line_segment::{LineSegment, LineSegmentId},
        station::{Station, StationId},
    };

    #[test]
    fn operation_add_and_delete_line_matches_model_behavior() {
        let mut root = DiagramRoot::default();
        let line_id = LineId::new(WeaverailId::new(1));
        let line = Line::new(line_id, "神明線", &[]);

        assert!(add_line(&mut root, line.clone()).is_ok());
        assert_eq!(root.lines.len(), 1);

        let removed = delete_line(&mut root, line_id);
        assert!(removed.is_ok());
        assert_eq!(removed.unwrap().id, line_id);
    }

    #[test]
    fn operation_validate_line_matches_model_behavior() {
        let mut root = DiagramRoot::default();
        let start = StationId::new(WeaverailId::new(10));
        let end = StationId::new(WeaverailId::new(11));
        let line_id = LineId::new(WeaverailId::new(1));
        let segment_id = LineSegmentId::new(WeaverailId::new(2));

        root.add_station(Station::new(start, "梅田")).unwrap();
        root.add_station(Station::new(end, "大阪")).unwrap();
        root.add_segment(LineSegment::new(segment_id, start, end))
            .unwrap();
        add_line(&mut root, Line::new(line_id, "神明線", &[])).unwrap();

        assert!(validate_line(&root, line_id).is_ok());
        assert!(validate_line(&root, LineId::new(WeaverailId::new(99))).is_err());
    }
}
