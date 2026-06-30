use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::model::{line_segment::LineSegmentId, station::StationId, time::Time, train::TrainId};

#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ResultWeftTrain {
    pub train_id: TrainId,
    pub times: Vec<ResultWeftTime>,
}

#[derive(ts_rs::TS, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ResultWeftTime {
    pub train_id: TrainId,
    pub before_segment_id: Option<LineSegmentId>,
    pub next_segment_id: Option<LineSegmentId>,
    pub station_id: StationId,
    pub arrival_time: Option<Time>,
    pub departure_time: Option<Time>,
    pub stop_type: StopType,
}

#[derive(ts_rs::TS, Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum StopType {
    #[default]
    Stop,
    Pass,
}

/// グラフのノードの識別子を表す
#[derive(Clone, PartialEq, Default, Eq, Hash, Copy, Debug)]
pub struct NodeId(pub usize);
impl NodeId {
    pub fn new(value: usize) -> Self {
        Self(value)
    }
}

/// 列車時刻を計算するためのグラフのノードを表す
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WeftNode {
    /// ノードの識別子
    pub node_id: NodeId,
    /// 駅ID
    pub station_id: StationId,
    /// 列車ID
    pub train_id: TrainId,
    /// 駅間ID
    pub segment_id: LineSegmentId,
    /// 停車パターン
    pub stop_type: StopType,
    /// グラフへのエッジ
    pub edges: SmallVec<[(NodeId, Time); 2]>,
    /// ノードの種類
    pub node_type: NodeType,
}

#[derive(PartialEq, Clone, Debug, Eq, Hash, Copy, Default)]
pub enum NodeType {
    #[default]
    Arrival,
    Departure,
    Root,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct WeftTempObj {
    pub nodes: Vec<WeftNode>,
    pub lookup: FxHashMap<LookupNodeKey, usize>,
}

#[derive(Clone, PartialEq, Debug, Default)]
#[derive(Eq, Hash, Copy)]
pub struct LookupNodeKey(u64);
impl LookupNodeKey {
    pub fn new(train_id: TrainId, segment_id: LineSegmentId, node_type: NodeType) -> Self {
        let raw_train_id = train_id.0.0;
        let raw_segment_id = segment_id.0.0;
        let raw_node_type = match node_type {
            NodeType::Arrival => 1,
            NodeType::Departure => 2,
            NodeType::Root => 3,
        };
        Self((raw_node_type as u64) << 62 | (raw_segment_id as u64) << 31 | raw_train_id as u64)
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct WeftTempStore {
    pub nodes: WeftTempObj,
    pub node_array: Vec<usize>,
    pub times: Vec<Time>,
    pub version: u32,
}
