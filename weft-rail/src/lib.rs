pub(crate) mod make_node;
pub(crate) mod ripple;
pub(crate) mod sort;
pub(crate) mod time_result;
pub(crate) mod update_node;

use std::collections::HashMap;

use smallvec::SmallVec;
use weaverail_model::{model::{
    DiagramRoot, line_segment::LineSegmentId, station::StationId, time::Time,
    timetable::TimetableId, train::TrainId,
}, result_weft::{ResultWeftTrain, StopType}};

use crate::{
    make_node::{get_node_by_nodeid, make_node},
    ripple::ripple_time,
    sort::sort_node,
    time_result::get_time_result,
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
#[derive(Clone, Debug, Default)]
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

#[derive(PartialEq, Clone, Debug, Eq, Hash, Copy)]
#[derive(Default)]
pub(crate) enum NodeType {
    #[default]
    Arrival,
    Departure,
    Root,
}

pub fn weave(root: &DiagramRoot, timetable_id: TimetableId) -> Vec<ResultWeftTrain> {
    let start = std::time::Instant::now();
    let nodes: (WeftNode, HashMap<TrainId, Vec<WeftNode>>) = make_node(root, timetable_id);
    let converted_nodes: Vec<&WeftNode> = get_node_by_nodeid(&nodes.0, &nodes.1);
    let duration = start.elapsed();
    println!("make_node: {}ms", duration.as_millis());

    let start = std::time::Instant::now();
    let node_array: Vec<&WeftNode> = sort_node(&converted_nodes);
    let duration = start.elapsed();
    println!("sort_node: {}ms", duration.as_millis());

    let start = std::time::Instant::now();
    let times: Vec<Time> = ripple_time(&node_array);
    let duration = start.elapsed();
    println!("ripple_node: {}ms", duration.as_millis());

    let start = std::time::Instant::now();
    let result: Vec<ResultWeftTrain> = get_time_result(root, timetable_id, node_array, &times);
    let duration = start.elapsed();
    println!("get_time: {}ms", duration.as_millis());

    result
}

#[test]
fn weave_test() {
    use crate::update_node::UpdateType;
    use crate::update_node::update_node;
    use weaverail_model::model::train::Train;

    let test_data = weaverail_model::test_data::diagram_root::get_test_data();
    let timetable_id = test_data
        .root
        .timetables
        .values()
        .find(|_| true)
        .unwrap()
        .id;
    let timetable = test_data.root.timetables.get(&timetable_id).unwrap();

    let start = std::time::Instant::now();
    let mut nodes: (WeftNode, HashMap<TrainId, Vec<WeftNode>>) =
        make_node(&test_data.root, timetable_id);
    let duration = start.elapsed();
    println!("make_node: {}ms", duration.as_millis());
    let start = std::time::Instant::now();
    let converted_nodes: Vec<&WeftNode> = get_node_by_nodeid(&nodes.0, &nodes.1);
    let duration = start.elapsed();
    println!("get_node_by_nodeid: {}ms", duration.as_millis());

    let start = std::time::Instant::now();
    let node_array: Vec<&WeftNode> = sort_node(&converted_nodes);
    let duration = start.elapsed();
    println!("sort_node: {}ms", duration.as_millis());

    let start = std::time::Instant::now();
    let times: Vec<Time> = ripple_time(&node_array);
    let duration = start.elapsed();
    println!("ripple_node: {}ms", duration.as_millis());

    let start = std::time::Instant::now();
    let result: Vec<ResultWeftTrain> =
        get_time_result(&test_data.root, timetable_id, node_array, &times);
    let duration = start.elapsed();
    println!("get_time: {}ms", duration.as_millis());

    let mut trains: Vec<&Train> = test_data
        .root
        .trains
        .values()
        .filter(|train| train.timetable_id == timetable_id)
        .collect();
    trains.sort_by(|a, b| a.start_departure_time.cmp(&b.start_departure_time));

    let change_type: UpdateType = UpdateType::ChangeTrainOrder(
        vec![
            "東二見",
            "西二見",
            "阿閇",
            "別府",
            "浜の宮",
            "尾上の松",
            "高砂",
        ]
        .windows(2)
        .map(|v| {
            let start = v[0];
            let end = v[1];
            let segment = *&test_data.root.find_segment_by_name(start, end).segment_id;
            (segment, trains[1].id, trains[3].id)
        })
        .collect(),
    );
    let start = std::time::Instant::now();
    update_node(&timetable, &mut nodes, change_type);
    let converted_nodes: Vec<&WeftNode> = get_node_by_nodeid(&nodes.0, &nodes.1);
    let node_array: Vec<&WeftNode> = sort_node(&converted_nodes);
    let times: Vec<Time> = ripple_time(&node_array);
    let duration = start.elapsed();
    println!("update_node: {}us", duration.as_micros());

    // println!("{:?}", result);
}
