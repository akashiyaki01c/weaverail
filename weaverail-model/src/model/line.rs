//! Weaverail上の「路線」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
//! - Line (路線)
//!   - LineSegment (駅間)

use std::iter;

use indexmap::map::Entry;
use serde::{Deserialize, Serialize};

use crate::{
    error::ModelError, model::{
        DiagramRoot, ExtensionProperty, PropertiableObject, line_segment::{LineSegment, LineSegmentId}, station::{Station, StationId},
    }, weaverail_id,
};
use crate::path::Heddle;

weaverail_id!(LineId, "LIN_");

/// 駅間への参照を表す構造体
#[derive(weaverail_object::RnaObjectable, ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct SegmentRef {
    pub segment_id: LineSegmentId,
    pub is_reversed: bool,
}

/// Weaverail上の1つの路線を表す構造体
#[derive(weaverail_object::RnaObjectable, ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
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
