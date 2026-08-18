//! Weaverail上の「路線」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
//! - Line (路線)
//!   - LineSegment (駅間)

use std::iter;

use indexmap::map::Entry;
use serde::{Deserialize, Serialize};

use crate::path::Heddle;
use crate::{
    error::ModelError,
    model::{
        DiagramRoot, ExtensionProperty, PropertiableObject,
        line_segment::{LineSegment, LineSegmentId},
        station::{Station, StationId},
    },
    weaverail_id,
};

weaverail_id!(LineId, "LIN_");

/// 駅間への参照を表す構造体
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
pub struct SegmentRef {
    pub segment_id: LineSegmentId,
    pub is_reversed: bool,
}

/// Weaverail上の1つの路線を表す構造体
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
pub struct Line {
    /// 識別ID
    pub id: LineId,
    /// 路線名 (例: "神明線")
    pub name: String,
    /// 路線に所属する駅間リスト
    pub segments: Vec<SegmentRef>,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl Line {
    pub fn new(id: LineId, name: &str, stations: &[SegmentRef]) -> Self {
        Self {
            id,
            name: name.to_string(),
            segments: stations.into(),
            ..Default::default()
        }
    }

    /// 駅間リストを取得する関数
    /// 計算量は `O(segments.len())`
    pub fn segments<'a>(&self, root: &'a DiagramRoot) -> Result<Vec<&'a LineSegment>, ModelError> {
        self.segments
            .iter()
            .map(|id| {
                root.segments
                    .get(&id.segment_id)
                    .ok_or(ModelError::ObjectNotFound)
            })
            .collect()
    }

    /// 最初の駅間を取得する関数
    pub fn first_segment<'a>(
        &self,
        root: &'a DiagramRoot,
    ) -> Result<Option<(&'a LineSegment, bool)>, ModelError> {
        if let Some(segment_id) = self.segments.first() {
            if let Some(segment) = root.segments.get(&segment_id.segment_id) {
                Ok(Some((segment, segment_id.is_reversed)))
            } else {
                Err(ModelError::ObjectNotFound)
            }
        } else {
            Ok(None)
        }
    }

    /// 最後の駅間を取得する関数
    pub fn last_segment<'a>(
        &self,
        root: &'a DiagramRoot,
    ) -> Result<Option<(&'a LineSegment, bool)>, ModelError> {
        if let Some(segment_id) = self.segments.last() {
            if let Some(segment) = root.segments.get(&segment_id.segment_id) {
                Ok(Some((segment, segment_id.is_reversed)))
            } else {
                Err(ModelError::ObjectNotFound)
            }
        } else {
            Ok(None)
        }
    }

    /// 先頭の駅IDを返す関数
    pub fn first_station_id(&self, root: &DiagramRoot) -> Result<Option<StationId>, ModelError> {
        let segment = self.first_segment(root)?;
        if let Some(segment) = segment {
            if segment.1 {
                Ok(Some(segment.0.end_station))
            } else {
                Ok(Some(segment.0.start_station))
            }
        } else {
            Ok(None)
        }
    }

    /// 末尾の駅IDを返す関数
    pub fn last_station_id(&self, root: &DiagramRoot) -> Result<Option<StationId>, ModelError> {
        let segment = self.last_segment(root)?;
        if let Some(segment) = segment {
            if segment.1 {
                Ok(Some(segment.0.start_station))
            } else {
                Ok(Some(segment.0.end_station))
            }
        } else {
            Ok(None)
        }
    }
}

impl PropertiableObject for Line {
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

impl DiagramRoot {
    /// 路線を追加する関数
    /// 計算オーダは`O(1)`
    /// 既に同一IDの路線が存在している場合はエラーを返す
    pub fn add_line(&mut self, line: Line) -> Result<(), ModelError> {
        match self.lines.entry(line.id) {
            Entry::Vacant(entry) => {
                entry.insert(line);
                Ok(())
            }
            Entry::Occupied(_) => Err(ModelError::DuplicateKey),
        }
    }

    /// 路線を削除する関数
    /// 計算オーダは`O(1)`
    /// 指定IDの路線が存在しない場合はエラーを返す
    pub fn delete_line(&mut self, line_id: LineId) -> Result<Line, ModelError> {
        self.lines
            .shift_remove(&line_id)
            .ok_or(ModelError::ObjectNotFound)
    }

    /// 路線に所属する駅を取得する関数
    pub fn get_line_stations(&self, line: &Line) -> Result<Vec<&Station>, ModelError> {
        if line.segments.is_empty() {
            return Ok(Vec::new());
        }
        let first_segment = self
            .segments
            .get(
                &line
                    .segments
                    .first()
                    .ok_or(ModelError::ObjectNotFound)?
                    .segment_id,
            )
            .ok_or(ModelError::ObjectNotFound)?;
        let start_id = first_segment.start_station;
        let end_ids: Result<Vec<_>, ModelError> = line
            .segments
            .iter()
            .map(|segment_id| {
                let segment = self
                    .segments
                    .get(&segment_id.segment_id)
                    .ok_or(ModelError::ObjectNotFound)?;
                Ok(segment.end_station)
            })
            .collect();
        let end_ids = end_ids?;
        let result: Result<Vec<_>, _> = iter::once(start_id)
            .chain(end_ids)
            .map(|station_id| {
                self.stations
                    .get(&station_id)
                    .ok_or(ModelError::ObjectNotFound)
            })
            .collect();
        result
    }

    /// SegmentIdから駅間を取得する関数
    pub fn get_segment(&self, segment_id: LineSegmentId) -> Option<&LineSegment> {
        self.segments
            .values()
            .find(|segment| segment.id == segment_id)
    }

    /// 路線の末尾に駅間を追加する関数
    pub fn push_back_line_segment(
        &mut self,
        line_id: LineId,
        segment_id: LineSegmentId,
        is_reversed: bool,
    ) -> Result<(), ModelError> {
        let (start_station_id, end_station_id) = {
            let segment = self
                .segments
                .get(&segment_id)
                .ok_or(ModelError::ObjectNotFound)?;
            (segment.start_station, segment.end_station)
        };
        let line: &Line = self.lines.get(&line_id).ok_or(ModelError::ObjectNotFound)?;
        let last_station = line.last_station_id(self)?;
        let line: &mut Line = self
            .lines
            .get_mut(&line_id)
            .ok_or(ModelError::ObjectNotFound)?;

        if let Some(last_station) = last_station {
            let is_valid = if is_reversed {
                end_station_id == last_station
            } else {
                start_station_id == last_station
            };
            if !is_valid {
                return Err(ModelError::Error);
            }
        }
        line.segments.push(SegmentRef {
            segment_id,
            is_reversed,
        });
        Ok(())
    }

    /// 路線の先頭に駅間を追加する関数
    pub fn push_front_line_segment(
        &mut self,
        line_id: LineId,
        segment_id: LineSegmentId,
        is_reversed: bool,
    ) -> Result<(), ModelError> {
        let (start_station_id, end_station_id) = {
            let segment = self
                .segments
                .get(&segment_id)
                .ok_or(ModelError::ObjectNotFound)?;
            (segment.start_station, segment.end_station)
        };
        let line: &Line = self.lines.get(&line_id).ok_or(ModelError::ObjectNotFound)?;
        let first_station = line.first_station_id(self)?;
        let line: &mut Line = self
            .lines
            .get_mut(&line_id)
            .ok_or(ModelError::ObjectNotFound)?;

        if let Some(last_station) = first_station {
            let is_valid = if is_reversed {
                start_station_id == last_station
            } else {
                end_station_id == last_station
            };
            if !is_valid {
                return Err(ModelError::Error);
            }
        }
        line.segments.insert(
            0,
            SegmentRef {
                segment_id,
                is_reversed,
            },
        );
        Ok(())
    }

    /// 路線の末尾の駅間を削除する関数
    pub fn pop_back_line_segment(&mut self, line_id: LineId) -> Result<SegmentRef, ModelError> {
        let line = self
            .lines
            .get_mut(&line_id)
            .ok_or(ModelError::ObjectNotFound)?;
        line.segments.pop().ok_or(ModelError::Empty)
    }

    /// 路線の先頭の駅間を削除する関数
    pub fn pop_front_line_segment(&mut self, line_id: LineId) -> Result<SegmentRef, ModelError> {
        let line = self
            .lines
            .get_mut(&line_id)
            .ok_or(ModelError::ObjectNotFound)?;
        if line.segments.is_empty() {
            return Err(ModelError::Empty);
        }
        Ok(line.segments.remove(0))
    }

    /// 駅間を、開始/終了駅名から検索する関数
    pub fn find_segment_by_name(
        &self,
        start_station_name: &str,
        end_station_name: &str,
    ) -> Result<&SegmentRef, ModelError> {
        let start_station = self
            .find_station_by_name(start_station_name)
            .ok_or(ModelError::ObjectNotFound)?
            .id;
        let end_station = self
            .find_station_by_name(end_station_name)
            .ok_or(ModelError::ObjectNotFound)?
            .id;
        let mut reversed_segment = None;

        for segment_ref in self.lines.values().flat_map(|line| &line.segments) {
            let segment = self
                .segments
                .get(&segment_ref.segment_id)
                .ok_or(ModelError::ObjectNotFound)?;

            if segment.start_station == start_station && segment.end_station == end_station {
                reversed_segment = Some(segment_ref);
                break;
            }
        }

        let mut forward_segment = None;
        for segment_ref in self.lines.values().flat_map(|line| &line.segments) {
            let segment = self
                .segments
                .get(&segment_ref.segment_id)
                .ok_or(ModelError::ObjectNotFound)?;

            if segment.start_station == end_station && segment.end_station == start_station {
                forward_segment = Some(segment_ref);
                break;
            }
        }
        if let Some(forward_segment) = forward_segment {
            Ok(forward_segment)
        } else {
            Ok(reversed_segment.ok_or(ModelError::ObjectNotFound)?)
        }
    }

    /// 路線データが正常な値であるかを検証する
    pub fn validate_line(&self, line_id: LineId) -> Result<(), ModelError> {
        let line = self.lines.get(&line_id).ok_or(ModelError::ObjectNotFound)?;
        for seg in &line.segments {
            let _ = self
                .get_segment(seg.segment_id)
                .ok_or(ModelError::ObjectNotFound)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::id::WeaverailId;

    /// Line の生成と基本プロパティが正しく設定されることをテスト
    #[test]
    fn test_line_creation() {
        let line_id = LineId::new(WeaverailId::new(1));
        let line = Line::new(line_id, "神明線", &[]);

        assert_eq!(line.id, line_id);
        assert_eq!(line.name, "神明線");
        assert_eq!(line.segments.len(), 0);
        assert_eq!(line.properties, ExtensionProperty::new());
    }

    /// Line の名前が正しく設定・変更されることをテスト
    #[test]
    fn test_line_name_change() {
        let line_id = LineId::new(WeaverailId::new(1));
        let mut line = Line::new(line_id, "神明線", &[]);

        assert_eq!(line.name, "神明線");

        line.name = "七夕線".to_string();
        assert_eq!(line.name, "七夕線");
    }

    /// DiagramRoot に Line を追加・削除できることをテスト
    #[test]
    fn test_add_and_delete_line() {
        let mut root = DiagramRoot::default();
        let line_id = LineId::new(WeaverailId::new(1));
        let line = Line::new(line_id, "神明線", &[]);

        // 追加テスト
        assert!(root.add_line(line.clone()).is_ok());
        assert_eq!(root.lines.len(), 1);
        assert_eq!(root.lines.get(&line_id).unwrap().name, "神明線");

        // 削除テスト
        let removed_line = root.delete_line(line_id);
        assert!(removed_line.is_ok());
        assert_eq!(removed_line.unwrap().name, "神明線");
        assert_eq!(root.lines.len(), 0);
    }

    /// 同一IDの Line を2つ追加しようとするとエラーになることをテスト
    #[test]
    fn test_duplicate_line_id_error() {
        let mut root = DiagramRoot::default();
        let line_id = LineId::new(WeaverailId::new(1));
        let line1 = Line::new(line_id, "神明線", &[]);
        let line2 = Line::new(line_id, "七夕線", &[]);

        assert!(root.add_line(line1).is_ok());

        // 同一IDで追加しようとする
        let result = root.add_line(line2);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ModelError::DuplicateKey);
        assert_eq!(root.lines.len(), 1);
    }

    /// 存在しない Line ID を削除しようとするとエラーになることをテスト
    #[test]
    fn test_delete_nonexistent_line() {
        let mut root = DiagramRoot::default();
        let line_id = LineId::new(WeaverailId::new(1));

        let result = root.delete_line(line_id);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ModelError::ObjectNotFound);
    }

    /// Line の拡張プロパティを取得・設定・削除できることをテスト
    #[test]
    fn test_line_properties() {
        let line_id = LineId::new(WeaverailId::new(1));
        let mut line = Line::new(line_id, "神明線", &[]);

        // プロパティを設定
        let value = Heddle::String("local".to_string());
        let result = line.set_property("line_type", value.clone());
        assert!(result.is_none());

        // プロパティを取得
        let retrieved = line.get_property("line_type");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), &value);

        // プロパティを削除
        let removed = line.remove_property("line_type");
        assert!(removed.is_some());
        assert!(line.get_property("line_type").is_none());
    }

    /// SegmentRef の生成と基本プロパティが正しく設定されることをテスト
    #[test]
    fn test_segment_ref_creation() {
        let segment_id = LineSegmentId::new(WeaverailId::new(1));
        let segment_ref = SegmentRef {
            segment_id,
            is_reversed: false,
        };

        assert_eq!(segment_ref.segment_id, segment_id);
        assert!(!segment_ref.is_reversed);
    }

    /// SegmentRef の reverse フラグが正しく設定できることをテスト
    #[test]
    fn test_segment_ref_reversed() {
        let segment_id = LineSegmentId::new(WeaverailId::new(1));
        let mut segment_ref = SegmentRef {
            segment_id,
            is_reversed: false,
        };

        assert!(!segment_ref.is_reversed);

        segment_ref.is_reversed = true;
        assert!(segment_ref.is_reversed);
    }

    /// 複数の Line を追加・管理できることをテスト
    #[test]
    fn test_multiple_lines() {
        let mut root = DiagramRoot::default();
        let mut line_ids = Vec::new();

        // 5つの路線を追加
        for i in 1..=5 {
            let line_id = LineId::new(WeaverailId::new(i));
            let line = Line::new(line_id, &format!("路線{}", i), &[]);
            line_ids.push(line_id);
            assert!(root.add_line(line).is_ok());
        }

        assert_eq!(root.lines.len(), 5);

        // 全路線を削除
        for line_id in line_ids {
            assert!(root.delete_line(line_id).is_ok());
        }

        assert_eq!(root.lines.len(), 0);
    }

    /// Line が空の segments を持つ場合、first_segment は None を返す
    #[test]
    fn test_first_segment_empty() {
        let root = DiagramRoot::default();
        let line_id = LineId::new(WeaverailId::new(1));
        let line = Line::new(line_id, "神明線", &[]);

        let result = line.first_segment(&root);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    /// Line が空の segments を持つ場合、last_segment は None を返す
    #[test]
    fn test_last_segment_empty() {
        let root = DiagramRoot::default();
        let line_id = LineId::new(WeaverailId::new(1));
        let line = Line::new(line_id, "神明線", &[]);

        let result = line.last_segment(&root);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
