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
