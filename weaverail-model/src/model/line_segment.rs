use indexmap::map::Entry;
use serde::{Deserialize, Serialize};

use crate::path::Heddle;
use crate::{
    error::ModelError,
    model::{
        DiagramRoot, ExtensionProperty, PropertiableObject,
        station::{Station, StationId},
    },
    weaverail_id,
};

weaverail_id!(LineSegmentId, "SGM_");

/// Weaverail上の1つの路線に属する駅間を表す構造体
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
pub struct LineSegment {
    /// 識別ID
    pub id: LineSegmentId,
    /// 開始駅ID
    pub start_station: StationId,
    /// 終了駅ID
    pub end_station: StationId,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl LineSegment {
    pub fn new(id: LineSegmentId, start_station: StationId, end_station: StationId) -> Self {
        Self {
            id,
            start_station,
            end_station,
            ..Default::default()
        }
    }

    /// 開始駅を取得する関数
    /// 計算量は `O(1)`
    pub fn start_station<'a>(&self, root: &'a DiagramRoot) -> Result<&'a Station, ModelError> {
        root.stations
            .get(&self.start_station)
            .ok_or(ModelError::ObjectNotFound)
    }

    /// 終了駅を取得する関数
    /// 計算量は `O(1)`
    pub fn end_station<'a>(&self, root: &'a DiagramRoot) -> Result<&'a Station, ModelError> {
        root.stations
            .get(&self.end_station)
            .ok_or(ModelError::ObjectNotFound)
    }

    /// 駅間が指定駅を参照しているか
    pub fn contains_station(&self, station_id: StationId) -> bool {
        self.start_station == station_id || self.end_station == station_id
    }
}
impl DiagramRoot {
    /// 駅間を追加する関数
    /// 計算オーダは `O(1)`
    /// 既に同一IDの駅間が存在している場合はエラーを返す
    pub fn add_segment(&mut self, segment: LineSegment) -> Result<(), ModelError> {
        match self.segments.entry(segment.id) {
            Entry::Vacant(entry) => {
                entry.insert(segment);
                Ok(())
            }
            Entry::Occupied(_) => Err(ModelError::DuplicateKey),
        }
    }

    /// 駅間を削除する関数
    /// 計算オーダは `O(segments.len + template_trains.len)`
    /// 指定IDの駅が存在しない場合はエラーを返す
    /// 路線から参照されている場合はエラーを返す
    /// テンプレート列車から参照されている場合はエラーを返す
    pub fn delete_segment(&mut self, segment_id: LineSegmentId) -> Result<LineSegment, ModelError> {
        if self
            .lines
            .values()
            .any(|line| line.segments.iter().any(|seg| seg.segment_id == segment_id))
        {
            return Err(ModelError::ExternalReferenced);
        }
        if self
            .template_trains
            .values()
            .any(|train| train.contains_segment(segment_id))
        {
            return Err(ModelError::ExternalReferenced);
        }

        self.segments
            .shift_remove(&segment_id)
            .ok_or(ModelError::ObjectNotFound)
    }

    /// 駅間データが正常な値であるかを検証する
    pub fn validate_segment(&self, segment_id: LineSegmentId) -> Result<(), ModelError> {
        let segment = self
            .get_segment(segment_id)
            .ok_or(ModelError::ObjectNotFound)?;
        let _ = segment.start_station(self)?;
        let _ = segment.end_station(self)?;

        Ok(())
    }
}
impl PropertiableObject for LineSegment {
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
        line::{Line, LineId, SegmentRef},
        station::Station,
    };

    #[test]
    fn test_line_segment_creation() {
        let segment_id = LineSegmentId::new(WeaverailId::new(1));
        let start_station = StationId::new(WeaverailId::new(10));
        let end_station = StationId::new(WeaverailId::new(11));

        let segment = LineSegment::new(segment_id, start_station, end_station);

        assert_eq!(segment.id, segment_id);
        assert_eq!(segment.start_station, start_station);
        assert_eq!(segment.end_station, end_station);
        assert_eq!(segment.properties, ExtensionProperty::new());
    }

    #[test]
    fn test_add_and_delete_segment() {
        let mut root = DiagramRoot::default();
        let start_station = StationId::new(WeaverailId::new(10));
        let end_station = StationId::new(WeaverailId::new(11));
        let segment_id = LineSegmentId::new(WeaverailId::new(1));

        root.add_station(Station::new(start_station, "梅田"))
            .unwrap();
        root.add_station(Station::new(end_station, "大阪")).unwrap();

        let segment = LineSegment::new(segment_id, start_station, end_station);
        assert!(root.add_segment(segment.clone()).is_ok());
        assert_eq!(root.segments.len(), 1);

        let removed = root.delete_segment(segment_id);
        assert!(removed.is_ok());
        assert_eq!(removed.unwrap().id, segment_id);
        assert_eq!(root.segments.len(), 0);
    }

    #[test]
    fn test_duplicate_segment_id_error() {
        let mut root = DiagramRoot::default();
        let start_station = StationId::new(WeaverailId::new(10));
        let end_station = StationId::new(WeaverailId::new(11));
        let segment_id = LineSegmentId::new(WeaverailId::new(1));

        root.add_station(Station::new(start_station, "梅田"))
            .unwrap();
        root.add_station(Station::new(end_station, "大阪")).unwrap();

        let segment1 = LineSegment::new(segment_id, start_station, end_station);
        let segment2 = LineSegment::new(segment_id, end_station, start_station);

        assert!(root.add_segment(segment1).is_ok());
        let result = root.add_segment(segment2);
        assert_eq!(result.unwrap_err(), ModelError::DuplicateKey);
    }

    #[test]
    fn test_delete_nonexistent_segment() {
        let mut root = DiagramRoot::default();
        let segment_id = LineSegmentId::new(WeaverailId::new(1));

        let result = root.delete_segment(segment_id);
        assert_eq!(result.unwrap_err(), ModelError::ObjectNotFound);
    }

    #[test]
    fn test_segment_contains_station() {
        let start_station = StationId::new(WeaverailId::new(10));
        let end_station = StationId::new(WeaverailId::new(11));
        let segment = LineSegment::new(
            LineSegmentId::new(WeaverailId::new(1)),
            start_station,
            end_station,
        );

        assert!(segment.contains_station(start_station));
        assert!(segment.contains_station(end_station));
        assert!(!segment.contains_station(StationId::new(WeaverailId::new(99))));
    }

    #[test]
    fn test_validate_segment() {
        let mut root = DiagramRoot::default();
        let start_station = StationId::new(WeaverailId::new(10));
        let end_station = StationId::new(WeaverailId::new(11));
        let segment_id = LineSegmentId::new(WeaverailId::new(1));

        root.add_station(Station::new(start_station, "梅田"))
            .unwrap();
        root.add_station(Station::new(end_station, "大阪")).unwrap();
        root.add_segment(LineSegment::new(segment_id, start_station, end_station))
            .unwrap();

        assert!(root.validate_segment(segment_id).is_ok());
        assert!(
            root.validate_segment(LineSegmentId::new(WeaverailId::new(2)))
                .is_err()
        );
    }

    #[test]
    fn test_segment_properties() {
        let mut segment = LineSegment::new(
            LineSegmentId::new(WeaverailId::new(1)),
            StationId::new(WeaverailId::new(10)),
            StationId::new(WeaverailId::new(11)),
        );
        let value = Heddle::String("priority".to_string());

        assert!(segment.set_property("ranking", value.clone()).is_none());
        assert_eq!(segment.get_property("ranking").unwrap(), &value);
        assert!(segment.remove_property("ranking").is_some());
        assert!(segment.get_property("ranking").is_none());
    }

    #[test]
    fn test_delete_segment_referenced_by_line_is_rejected() {
        let mut root = DiagramRoot::default();
        let start_station = StationId::new(WeaverailId::new(10));
        let end_station = StationId::new(WeaverailId::new(11));
        let segment_id = LineSegmentId::new(WeaverailId::new(1));
        let line_id = LineId::new(WeaverailId::new(2));

        root.add_station(Station::new(start_station, "梅田"))
            .unwrap();
        root.add_station(Station::new(end_station, "大阪")).unwrap();
        root.add_segment(LineSegment::new(segment_id, start_station, end_station))
            .unwrap();
        root.add_line(Line::new(
            line_id,
            "大阪線",
            &[SegmentRef {
                segment_id,
                is_reversed: false,
            }],
        ))
        .unwrap();

        let result = root.delete_segment(segment_id);
        assert_eq!(result.unwrap_err(), ModelError::ExternalReferenced);
    }
}
