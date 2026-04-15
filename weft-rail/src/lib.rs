pub(crate) mod make_node;
pub(crate) mod ripple;
pub(crate) mod sort;
pub(crate) mod time_result;

use std::collections::HashMap;

use smallvec::SmallVec;
use weaverail_model::model::{
    DiagramRoot, line::LineSegmentId, station::StationId, time::Time, timetable::TimetableId,
    train::TrainId,
};

use crate::{
    make_node::make_node, ripple::ripple_time, sort::sort_node, time_result::get_time_result,
};

/// グラフのノードの識別子を表す
#[derive(Clone, PartialEq, Default, Eq, Hash, Copy, Debug)]
pub(crate) struct NodeId(usize);
impl NodeId {
    pub fn new(value: usize) -> Self {
        Self(value)
    }
}

/// 列車時刻を計算するためのグラフのノードを表す
#[derive(Clone, Debug)]
pub(crate) struct WeftNode {
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

#[derive(Clone, Copy, Debug)]
pub enum StopType {
    Stop,
    Pass,
}

#[derive(PartialEq, Clone, Debug)]
pub(crate) enum NodeType {
    Arrival,
    Departure,
    Root,
}

#[derive(Clone, Debug)]
pub struct ResultWeftTrain {
    pub train_id: TrainId,
    pub times: Vec<ResultWeftTime>,
}

#[derive(Clone, Debug)]
pub struct ResultWeftTime {
    pub train_id: TrainId,
    pub before_segment_id: Option<LineSegmentId>,
    pub next_segment_id: Option<LineSegmentId>,
    pub station_id: StationId,
    pub arrival_time: Option<Time>,
    pub departure_time: Option<Time>,
    pub stop_type: StopType,
}

pub fn weave(root: &DiagramRoot, timetable_id: TimetableId) -> Vec<ResultWeftTrain> {
    let nodes: HashMap<NodeId, WeftNode> = make_node(root, timetable_id);
    let nodes: Vec<&WeftNode> = sort_node(&nodes);
    let times: Vec<Time> = ripple_time(&nodes);
    let result: Vec<ResultWeftTrain> = get_time_result(root, timetable_id, nodes, &times);
    result
}

#[test]
fn weave_test() {
    let test_data = weaverail_model::test_data::diagram_root::get_test_data();
    let timetable_id = test_data
        .root
        .timetables
        .values()
        .find(|_| true)
        .unwrap()
        .id;
    let result = weave(&test_data.root, timetable_id);
    println!("{:?}", result);
}
