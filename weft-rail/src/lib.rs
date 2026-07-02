pub mod make_node;
pub mod make_node_diff;
pub mod ripple;
pub mod ripple_diff;
pub mod sort;
pub mod sort_diff;
pub mod time_result;
pub mod time_result_diff;
pub mod update_node;

use weaverail_model::{
    model::{DiagramRoot, time::Time, timetable::TimetableId},
    result_weft::{ResultWeftTrain, WeftNode, WeftTempObj},
};

use crate::time_result_diff::get_time_result_diff;

pub fn weave(root: &DiagramRoot, timetable_id: TimetableId) -> Vec<ResultWeftTrain> {
    let start = std::time::Instant::now();
    let nodes: WeftTempObj = make_node_diff::make_node(root, timetable_id);
    let duration = start.elapsed();
    println!("make_node: {}us", duration.as_micros());

    let start = std::time::Instant::now();
    let node_array: Vec<usize> = sort_diff::sort_node(&nodes);
    let duration = start.elapsed();
    println!("sort_node: {}us", duration.as_micros());

    let start = std::time::Instant::now();
    let time: Vec<Time> = ripple_diff::ripple_node_diff(&nodes, &node_array);
    let duration = start.elapsed();
    println!("ripple_node: {}us", duration.as_micros());

    let start = std::time::Instant::now();
    let result: Vec<ResultWeftTrain> = get_time_result_diff(root, timetable_id, &nodes, &time);
    let duration = start.elapsed();
    println!("get_time: {}us", duration.as_micros());

    result
}

#[test]
fn weave_test() {
    use crate::make_node::{get_node_by_nodeid, make_node};
    use crate::ripple::ripple_time;
    use crate::sort::sort_node;
    use crate::time_result::get_time_result;
    use crate::update_node::UpdateType;
    use crate::update_node::update_node;
    use std::collections::HashMap;
    use weaverail_model::model::train::Train;
    use weaverail_model::model::train::TrainId;

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
            (false, segment, trains[1].id, trains[3].id)
        })
        .collect(),
    );
    let start = std::time::Instant::now();
    update_node(&timetable, &mut nodes, change_type);
    let converted_nodes: Vec<&WeftNode> = get_node_by_nodeid(&nodes.0, &nodes.1);
    let node_array: Vec<&WeftNode> = sort_node(&converted_nodes);
    let _times: Vec<Time> = ripple_time(&node_array);
    let duration = start.elapsed();
    println!("update_node: {}us", duration.as_micros());

    println!("{:?}", result);
}

#[test]
fn weave_test_diff() {
    let test_data = weaverail_model::test_data::diagram_root::get_test_data();
    let timetable_id = test_data
        .root
        .timetables
        .values()
        .find(|_| true)
        .unwrap()
        .id;
    let _timetable = test_data.root.timetables.get(&timetable_id).unwrap();

    let start = std::time::Instant::now();
    let nodes: WeftTempObj = make_node_diff::make_node(&test_data.root, timetable_id);
    let duration = start.elapsed();
    println!("make_node: {}us", duration.as_micros());

    let start = std::time::Instant::now();
    let node_array: Vec<usize> = sort_diff::sort_node(&nodes);
    let duration = start.elapsed();
    println!("sort_node: {}us", duration.as_micros());

    let start = std::time::Instant::now();
    let time: Vec<Time> = ripple_diff::ripple_node_diff(&nodes, &node_array);
    let duration = start.elapsed();
    println!("ripple_node: {}us", duration.as_micros());

    println!("{:?}", time);

    let start = std::time::Instant::now();
    let _result: Vec<ResultWeftTrain> =
        get_time_result_diff(&test_data.root, timetable_id, &nodes, &time);
    let duration = start.elapsed();
    println!("get_time: {}us", duration.as_micros());

    // println!("{:?}", result);
}

#[test]
fn make_node_test() {
    use std::collections::HashMap;
    use weaverail_model::model::train::TrainId;

    // 前処理 開始
    let test_data = weaverail_model::test_data::diagram_root::get_test_data();
    let timetable_id = test_data
        .root
        .timetables
        .values()
        .find(|_| true)
        .unwrap()
        .id;
    let _timetable = test_data.root.timetables.get(&timetable_id).unwrap();
    // 前処理 終了

    let nodes: (WeftNode, HashMap<TrainId, Vec<WeftNode>>) =
        make_node::make_node(&test_data.root, timetable_id);
    let converted_nodes: Vec<&WeftNode> = make_node::get_node_by_nodeid(&nodes.0, &nodes.1);
    let node_array: Vec<&WeftNode> = sort::sort_node(&converted_nodes);

    let diff_nodes: WeftTempObj = make_node_diff::make_node(&test_data.root, timetable_id);
    let diff_node_array: Vec<usize> = sort_diff::sort_node(&diff_nodes);

    assert!(node_array.len() == diff_node_array.len());

    for i in 0..node_array.len() {
        let node = node_array[i];
        let diff_node = &diff_nodes.nodes[diff_node_array[i]];
        assert!(node.node_id == diff_node.node_id);
    }

    let ripple: Vec<Time> = ripple::ripple_time(&node_array);
    let diff_ripple = ripple_diff::ripple_node_diff(&diff_nodes, &diff_node_array);

    assert!(ripple.len() == diff_ripple.len());
    for i in 0..ripple.len() {
        assert!(ripple[i] == diff_ripple[i]);
    }

    let result: Vec<ResultWeftTrain> =
        time_result::get_time_result(&test_data.root, timetable_id, node_array, &ripple);
    let diff_result: Vec<ResultWeftTrain> =
        get_time_result_diff(&test_data.root, timetable_id, &diff_nodes, &diff_ripple);

    assert!(result.len() == diff_result.len());
    for i in 0..result.len() {
        let train = &result[i];
        let diff_train = &diff_result[i];
        assert!(train.train_id == diff_train.train_id);
        assert!(train.times.len() == diff_train.times.len());
        for j in 0..train.times.len() {
            let time = &train.times[j];
            let diff_time = &diff_train.times[j];
            assert!(time.arrival_time == diff_time.arrival_time);
            assert!(time.departure_time == diff_time.departure_time);
        }
    }
}
