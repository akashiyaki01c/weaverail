//! Weaverail上の「路線」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
//! - Line (路線)
//!   - LineSegment (駅間)

use std::collections::hash_map::Entry;
use std::iter;

use serde::{Deserialize, Serialize};

use crate::{
    command::CommandError,
    error::ModelError,
    model::{
        DiagramRoot, ExtensionProperty,
        line_segment::{LineSegment, LineSegmentId},
        station::Station,
    },
    weaverail_id,
};

weaverail_id!(LineId, "LIN_");

/// Weaverail上の1つの路線を表す構造体
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Line {
    /// 識別ID
    pub id: LineId,
    /// 路線名 (例: "神明線")
    pub name: String,
    /// 路線に所属する駅間リスト
    pub segments: Vec<LineSegmentId>,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl Line {
    pub fn new(id: LineId, name: &str, stations: &[LineSegmentId]) -> Self {
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
                    .get(id)
                    .ok_or_else(|| ModelError::ObjectNotFound)
            })
            .collect()
    }

    /// 最初の駅間を取得する関数
    pub fn first_segment<'a>(&self, root: &'a DiagramRoot) -> Result<Option<&'a LineSegment>, ModelError> {
        if let Some(segment_id) = self.segments.first() {
            if let Some(segment) = root.segments.get(segment_id) {
                Ok(Some(segment))
            } else {
                Err(ModelError::ObjectNotFound)
            }
        } else {
            Ok(None)
        }
    }

    /// 最後の駅間を取得する関数
    pub fn last_segment<'a>(&self, root: &'a DiagramRoot) -> Result<Option<&'a LineSegment>, ModelError> {
        if let Some(segment_id) = self.segments.last() {
            if let Some(segment) = root.segments.get(segment_id) {
                Ok(Some(segment))
            } else {
                Err(ModelError::ObjectNotFound)
            }
        } else {
            Ok(None)
        }
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
            .remove(&line_id)
            .ok_or(ModelError::ObjectNotFound)
    }

    /// 路線に所属する駅を取得する関数
    pub fn get_line_stations(&self, line: &Line) -> Vec<&Station> {
        if line.segments.is_empty() {
            return Vec::new();
        }
        let first_segment = self.segments.get(line.segments.first().unwrap()).unwrap();
        let start_id = first_segment.start_station;
        let end_ids = line.segments.iter().map(|segment_id| {
            let segment = self.segments.get(segment_id).unwrap();
            segment.end_station
        });
        iter::once(start_id)
            .chain(end_ids)
            .map(|station_id| self.stations.get(&station_id).unwrap())
            .collect()
    }

    /// SegmentIdから駅間を取得する関数
    pub fn get_segment(&self, segment_id: LineSegmentId) -> Option<&LineSegment> {
        self.segments
            .values()
            .find(|segment| segment.id == segment_id)
    }

    /// 路線の末尾に駅を追加する
    pub fn append_segment(
        &mut self,
        line_id: LineId,
        segment: LineSegmentId,
    ) -> Result<(), CommandError> {
        let line = self
            .lines
            .get_mut(&line_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        if line.segments.is_empty() {
            line.segments.push(segment)
        } else {
            let last_segment = self.segments.get(line.segments.last().unwrap()).unwrap();
            let segment = self.segments.get(&segment).unwrap();
            let segment_end_id = last_segment.end_station;
            if segment_end_id != segment.start_station {
                return Err(CommandError::Inconsistent);
            }
            line.segments.push(segment.id)
        }

        Ok(())
    }

    /// 駅間を、開始/終了駅名から検索する関数
    pub fn find_segment_by_name(
        &self,
        start_station_name: &str,
        end_station_name: &str,
    ) -> (&LineSegmentId, bool) {
        let start_station = self.find_station_by_name(start_station_name).expect("").id;
        let end_station = self.find_station_by_name(end_station_name).expect("").id;
        let reversed_segment =
            self.lines
                .values()
                .flat_map(|line| &line.segments)
                .find(|segment| {
                    let segment = self.segments.get(&segment).unwrap();
                    segment.start_station == start_station && segment.end_station == end_station
                });
        let forward_segment = self
            .lines
            .values()
            .flat_map(|line| &line.segments)
            .find(|segment| {
                let segment = self.segments.get(&segment).unwrap();
                segment.start_station == end_station && segment.end_station == start_station
            });
        if forward_segment.is_some() {
            (forward_segment.unwrap(), false)
        } else {
            (reversed_segment.unwrap(), false)
        }
    }
}
