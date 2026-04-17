//! Weaverail上の「路線」を表すデータ構造を定義するモジュールであり、以下のモデルの定義を内包する
//! - Line (路線)
//!   - LineSegment (駅間)

use std::collections::hash_map::Entry;
use std::iter;

use serde::{Deserialize, Serialize};

use crate::{
    command::CommandError, model::{
        DiagramRoot, ExtensionProperty,
        station::{Station, StationId},
    }, weaverail_id
};

weaverail_id!(LineId, "LINE");

/// Weaverail上の1つの路線を表す構造体
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Line {
    /// 識別ID
    pub id: LineId,
    /// 路線名 (例: "神明線")
    pub name: String,
    /// 路線に所属する駅間リスト
    pub segments: Vec<LineSegment>,
    /// 拡張プロパティ
    pub properties: ExtensionProperty,
}
impl Line {
    pub fn new(id: LineId, name: &str, stations: &[LineSegment]) -> Self {
        Self {
            id,
            name: name.to_string(),
            segments: stations.into(),
            ..Default::default()
        }
    }

    /// 路線が指定駅を参照しているか
    pub fn contains_station(&self, station_id: StationId) -> bool {
        self.segments
            .iter()
            .any(|segment| segment.contains_station(station_id))
    }
}

weaverail_id!(LineSegmentId, "SEGM");

/// Weaverail上の1つの路線に属する駅間を表す構造体
#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
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

    /// 駅間が指定駅を参照しているか
    pub fn contains_station(&self, station_id: StationId) -> bool {
        self.start_station == station_id || self.end_station == station_id
    }
}

impl DiagramRoot {
    /// 路線を追加する関数
    /// 既に同一IDの路線が存在している場合はエラーを返す
    pub fn add_line(&mut self, line: Line) -> Result<(), CommandError> {
        match self.lines.entry(line.id) {
            Entry::Vacant(entry) => {
                entry.insert(line);
                Ok(())
            }
            Entry::Occupied(_) => Err(CommandError::DuplicateKey),
        }
    }

    /// 路線を削除する関数
    /// 指定IDの路線が存在しない場合はエラーを返す
    /// テンプレート列車から参照されている場合はエラーを返す
    pub fn delete_line(&mut self, line_id: LineId) -> Result<Line, CommandError> {
        let line = self
            .lines
            .get(&line_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        let is_referenced = self
            .template_trains
            .values()
            .any(|train| train.contains_line(line));

        if is_referenced {
            return Err(CommandError::ExternalReference);
        }

        self.lines
            .remove(&line_id)
            .ok_or(CommandError::TargetObjectNotFound)
    }

    /// 路線に所属する駅を取得する関数
    pub fn get_line_stations(&self, line: &Line) -> Vec<&Station> {
        if line.segments.is_empty() {
            return Vec::new();
        }
        let start_id = line.segments.first().unwrap().start_station;
        let end_ids = line.segments.iter().map(|segment| segment.end_station);
        iter::once(start_id)
            .chain(end_ids)
            .map(|station_id| self.stations.get(&station_id).unwrap())
            .collect()
    }

    /// SegmentIdから駅間を取得する関数
    pub fn get_segment(&self, segment_id: LineSegmentId) -> Option<&LineSegment> {
        self.lines
            .values()
            .flat_map(|line| &line.segments)
            .find(|segment| segment.id == segment_id)
    }

    /// 路線の末尾に駅を追加する
    pub fn append_segment(
        &mut self,
        line_id: LineId,
        segment: LineSegment,
    ) -> Result<(), CommandError> {
        let line = self
            .lines
            .get_mut(&line_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        if line.segments.is_empty() {
            line.segments.push(segment)
        } else {
            let segment_end_id = line.segments.last().unwrap().end_station;
            if segment_end_id != segment.start_station {
                return Err(CommandError::Inconsistent);
            }
            line.segments.push(segment)
        }

        Ok(())
    }

    /// 駅間を、開始/終了駅名から検索する関数
    pub fn find_segment_by_name(
        &self,
        start_station_name: &str,
        end_station_name: &str,
    ) -> (&LineSegment, bool) {
        let start_station = self.find_station_by_name(start_station_name).expect("").id;
        let end_station = self.find_station_by_name(end_station_name).expect("").id;
        let reversed_segment =
            self.lines
                .values()
                .flat_map(|line| &line.segments)
                .find(|segment| {
                    segment.start_station == start_station && segment.end_station == end_station
                });
        let forward_segment = self
            .lines
            .values()
            .flat_map(|line| &line.segments)
            .find(|segment| {
                segment.start_station == end_station && segment.end_station == start_station
            });
        if forward_segment.is_some() {
            (forward_segment.unwrap(), false)
        } else {
            (reversed_segment.unwrap(), false)
        }
    }
}
