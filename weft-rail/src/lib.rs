pub(crate) mod make_node;

use smallvec::SmallVec;
use weaverail_model::model::{
    DiagramRoot, line::LineSegmentId, station::StationId, time::Time, timetable::TimetableId,
    train::TrainId,
};

use crate::make_node::make_node;

/// グラフのノードの識別子を表す
#[derive(Clone, PartialEq, Default, Eq, Hash, Copy)]
struct NodeId(usize);
impl NodeId {
    pub fn new(value: usize) -> Self {
        Self(value)
    }
}

/// 列車時刻を計算するためのグラフのノードを表す
struct WeftNode {
    /// ノードの識別子
    pub node_id: NodeId,
    /// 駅ID
    pub station_id: StationId,
    /// 列車ID
    pub train_id: TrainId,
    /// 駅間ID
    pub segment_id: LineSegmentId,
    /// グラフへのエッジ
    pub edges: SmallVec<[(NodeId, Time); 2]>,
    /// ノードの種類
    pub node_type: NodeType,
}

#[derive(PartialEq)]
enum NodeType {
    Arrival,
    Departure,
    Root,
}

pub fn weave(root: &DiagramRoot, timetable_id: TimetableId) {
    let nodes = make_node(root, timetable_id);
}
