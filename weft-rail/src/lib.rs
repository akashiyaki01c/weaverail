use std::collections::HashMap;

use petgraph::{
    algo::bellman_ford,
    graph::{DiGraph, NodeIndex},
};
use uuid::Uuid;
use weaverail_model::model::{
    DiagramRoot, template_train::StopType, time::Time, timetable::Timetable, train::Train,
    train_adjustment::TrainsAdjustmentType,
};

/// ノードの種類
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum GraphNodeType {
    /// 根ノード
    Root,
    /// 到着時刻
    ArrivalTime,
    /// 発車時刻
    DepartureTime,
}

/// ノードの識別子
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct GraphNodeId {
    /// 列車ID
    pub train_id: Uuid,
    /// 駅間ID
    pub segment_id: Uuid,
    /// 駅ID
    pub station_id: Uuid,
    /// ノードの種類
    pub node_type: GraphNodeType,
}
impl GraphNodeId {
    pub fn new(
        train_id: Uuid,
        segment_id: Uuid,
        station_id: Uuid,
        node_type: GraphNodeType,
    ) -> Self {
        Self {
            train_id,
            segment_id,
            station_id,
            node_type,
        }
    }
}

struct TrainWrapper<'a> {
    #[allow(dead_code)]
    pub train: &'a Train,
    pub nodes: Vec<(GraphNodeId, NodeIndex)>,
}
impl<'a> TrainWrapper<'a> {
    pub fn new(train: &'a Train) -> Self {
        Self {
            train,
            nodes: Vec::new(),
        }
    }
}

#[derive(Default)]
struct SegmentInfo {
    pub segment_id: Uuid,
    pub train_times_prograde: HashMap<Uuid, (Time, Time, Uuid)>,
    pub train_times_retrograde: HashMap<Uuid, (Time, Time, Uuid)>,
}

/// ダイヤグラムをグラフ理論によって生成するアルゴリズム
pub struct WeftGraph<'a> {
    /// ダイヤグラムデータ
    root: &'a DiagramRoot,
    timetable_id: Uuid,
}
impl<'a> WeftGraph<'a> {
    pub fn new(root: &'a DiagramRoot, timetable_id: Uuid) -> Self {
        Self { root, timetable_id }
    }

    fn timetable(&self) -> &Timetable {
        self.root
            .timetables
            .get(&self.timetable_id)
            .expect("timetables error")
    }

    pub fn weave(&self) -> bellman_ford::Paths<NodeIndex, f64> {
        let mut graph1 = DiGraph::<GraphNodeId, f64>::new();
        let mut nodes = HashMap::new();

        // 根ノード
        let root_node = {
            let node_id =
                GraphNodeId::new(Uuid::nil(), Uuid::nil(), Uuid::nil(), GraphNodeType::Root);
            let index = graph1.add_node(node_id.clone());
            nodes.insert(node_id.clone(), index);
            index
        };

        let conv_time = |time: Time| -> f64 { -Into::<f64>::into(time.total_second()) };
        let to_time = |time: f64| -> Time { Time::new_from_total_second(time as u32) };

        let mut train_wrappers = Vec::new();

        for train in self.timetable().trains.values() {
            let mut train_wrapper = TrainWrapper::new(&train);
            for template_segment in &train.template_segments {
                let template_train = self
                    .root
                    .template_trains
                    .get(&template_segment.template_train_id)
                    .expect("error");
                for (start, segment, end) in template_train.get_filtered_segment_iter(
                    template_segment.start_station_id,
                    template_segment.end_station_id,
                ) {
                    let start_node = GraphNodeId::new(
                        train.id,
                        segment.id,
                        start.station_id,
                        GraphNodeType::DepartureTime,
                    );
                    let start_node_index = {
                        let index = graph1.add_node(start_node.clone());
                        nodes.insert(start_node.clone(), index);
                        index
                    };

                    if train_wrapper.nodes.is_empty() {
                        graph1.add_edge(
                            root_node,
                            start_node_index,
                            conv_time(train.start_departure_time),
                        );
                    } else {
                        let last = train_wrapper.nodes.last().unwrap();
                        match start.stop_time {
                            StopType::Pass => graph1.add_edge(last.1, start_node_index, 0.0),
                            StopType::Stop(time) => {
                                graph1.add_edge(last.1, start_node_index, conv_time(time))
                            }
                        };
                    }
                    train_wrapper
                        .nodes
                        .push((start_node.clone(), start_node_index));

                    let end_node = GraphNodeId::new(
                        train.id,
                        segment.id,
                        end.station_id,
                        GraphNodeType::ArrivalTime,
                    );
                    let end_node_index = {
                        let index = graph1.add_node(end_node.clone());
                        nodes.insert(end_node.clone(), index);
                        index
                    };
                    {
                        let last = train_wrapper.nodes.last().unwrap();
                        graph1.add_edge(last.1, end_node_index, conv_time(segment.running_time));
                    }
                    train_wrapper.nodes.push((end_node.clone(), end_node_index));
                }
            }
            train_wrappers.push(train_wrapper);
        }

        // 待避情報
        for adjustment in self.timetable().adjustments.values() {
            match adjustment.adjustment {
                TrainsAdjustmentType::None => {}
                TrainsAdjustmentType::Waiting {
                    station_id,
                    waiting_train_id,
                    passing_train_id,
                    waiting_track_id: _,
                } => {
                    let waiting_arriving = train_wrappers
                        .iter()
                        .find(|wrapper| wrapper.train.id == waiting_train_id)
                        .unwrap()
                        .nodes
                        .iter()
                        .find(|node| {
                            node.0.station_id == station_id
                                && node.0.node_type == GraphNodeType::ArrivalTime
                        })
                        .unwrap()
                        .1;
                    let passing_arriving = train_wrappers
                        .iter()
                        .find(|wrapper| wrapper.train.id == passing_train_id)
                        .unwrap()
                        .nodes
                        .iter()
                        .find(|node| {
                            node.0.station_id == station_id
                                && node.0.node_type == GraphNodeType::ArrivalTime
                        })
                        .unwrap()
                        .1;
                    graph1.add_edge(
                        waiting_arriving,
                        passing_arriving,
                        conv_time(Time::new(0, 2, 0)),
                    );

                    let waiting_departure = train_wrappers
                        .iter()
                        .find(|wrapper| wrapper.train.id == waiting_train_id)
                        .unwrap()
                        .nodes
                        .iter()
                        .find(|node| {
                            node.0.station_id == station_id
                                && node.0.node_type == GraphNodeType::DepartureTime
                        })
                        .unwrap()
                        .1;
                    let passing_departure = train_wrappers
                        .iter()
                        .find(|wrapper| wrapper.train.id == passing_train_id)
                        .unwrap()
                        .nodes
                        .iter()
                        .find(|node| {
                            node.0.station_id == station_id
                                && node.0.node_type == GraphNodeType::DepartureTime
                        })
                        .unwrap()
                        .1;
                    graph1.add_edge(
                        passing_departure,
                        waiting_departure,
                        conv_time(Time::new(0, 2, 0)),
                    );
                }
            }
        }

        let res1: bellman_ford::Paths<NodeIndex, f64> = bellman_ford(&graph1, root_node).unwrap();

        let mut graph2 = graph1.clone();
        let mut segment_infos: Vec<SegmentInfo> = Vec::new();
        for segments in self.root.lines.values().flat_map(|line| &line.segments) {
            segment_infos.push(SegmentInfo {
                segment_id: segments.id,
                train_times_prograde: HashMap::new(),
                train_times_retrograde: HashMap::new(),
            });
        }
        for train in &train_wrappers {
            for template_segment in &train.train.template_segments {
                let template_train = self
                    .root
                    .template_trains
                    .get(&template_segment.template_train_id)
                    .expect("error");
                for (start, segment, end) in template_train.get_filtered_segment_iter(
                    template_segment.start_station_id,
                    template_segment.end_station_id,
                ) {
                    let info = segment_infos
                        .iter_mut()
                        .find(|info| info.segment_id == segment.segment_id)
                        .unwrap();
                    let departure_node_id = train
                        .nodes
                        .iter()
                        .find(|node| {
                            node.0.segment_id == segment.id
                                && node.0.node_type == GraphNodeType::DepartureTime
                        })
                        .unwrap();
                    let arrival_node_id = train
                        .nodes
                        .iter()
                        .find(|node| {
                            node.0.segment_id == segment.id
                                && node.0.node_type == GraphNodeType::ArrivalTime
                        })
                        .unwrap();
                    let departure_time = to_time(res1.distances[departure_node_id.1.index()]);
                    let arrival_time = to_time(res1.distances[arrival_node_id.1.index()]);
                    if segment.is_reversed {
                        info.train_times_retrograde
                            .insert(train.train.id, (departure_time, arrival_time, segment.id));
                    } else {
                        info.train_times_prograde
                            .insert(train.train.id, (departure_time, arrival_time, segment.id));
                    }
                }
            }
        }
        for info in &segment_infos {
            let mut sorted: Vec<(&Uuid, &(Time, Time, Uuid))> =
                info.train_times_prograde.iter().collect();
            sorted.sort_by_key(|&(_uuid, times)| times.0);
            for infos in sorted.windows(2) {
                let before_info = infos.get(0).unwrap();
                let current_info = infos.get(1).unwrap();
                let before_wrapper = train_wrappers
                    .iter()
                    .find(|wrapper| wrapper.train.id == *before_info.0)
                    .unwrap();
                let current_wrapper = train_wrappers
                    .iter()
                    .find(|wrapper| wrapper.train.id == *current_info.0)
                    .unwrap();
                let before_departure_node = before_wrapper
                    .nodes
                    .iter()
                    .find(|node| {
                        node.0.segment_id == before_info.1.2
                            && node.0.node_type == GraphNodeType::DepartureTime
                    })
                    .unwrap();
                let current_departure_node = current_wrapper
                    .nodes
                    .iter()
                    .find(|node| {
                        node.0.segment_id == current_info.1.2
                            && node.0.node_type == GraphNodeType::DepartureTime
                    })
                    .unwrap();
                let before_arrival_node = before_wrapper
                    .nodes
                    .iter()
                    .find(|node| {
                        node.0.segment_id == before_info.1.2
                            && node.0.node_type == GraphNodeType::ArrivalTime
                    })
                    .unwrap();
                let current_arrival_node = current_wrapper
                    .nodes
                    .iter()
                    .find(|node| {
                        node.0.segment_id == current_info.1.2
                            && node.0.node_type == GraphNodeType::ArrivalTime
                    })
                    .unwrap();

                /* if self
                    .timetable()
                    .adjustments
                    .values()
                    .find(|adjustment| match adjustment.adjustment {
                        TrainsAdjustmentType::None => false,
                        TrainsAdjustmentType::Waiting {
                            station_id,
                            waiting_track_id: _,
                            waiting_train_id: _,
                            passing_train_id: _,
                        } => before_departure_node.0.station_id == station_id,
                    })
                    .is_none() && before_info.1.0 + Time::new(0, 2, 0) > current_info.1.0
                {
                    graph2.add_edge(
                        before_departure_node.1,
                        current_departure_node.1,
                        conv_time(Time::new(0, 2, 0)),
                    );
                }
                if self
                    .timetable()
                    .adjustments
                    .values()
                    .find(|adjustment| match adjustment.adjustment {
                        TrainsAdjustmentType::None => false,
                        TrainsAdjustmentType::Waiting {
                            station_id,
                            waiting_track_id: _,
                            waiting_train_id: _,
                            passing_train_id: _,
                        } => before_arrival_node.0.station_id == station_id,
                    })
                    .is_none() && before_info.1.1 + Time::new(0, 2, 0) > current_info.1.1
                {
                    graph2.add_edge(
                        before_arrival_node.1,
                        current_arrival_node.1,
                        conv_time(Time::new(0, 2, 0)),
                    );
                } */
            }
        }

        let res2: bellman_ford::Paths<NodeIndex, f64> = bellman_ford(&graph2, root_node).unwrap();

        for train in &train_wrappers {
            for node in &train.nodes {
                let a = res2.distances[node.1.index()];
                let time = Time::new_from_total_second(-a as u32);
                let sta = self.root.stations.get(&node.0.station_id).unwrap();
                let ty = match node.0.node_type {
                    GraphNodeType::ArrivalTime => "着",
                    GraphNodeType::DepartureTime => "発",
                    GraphNodeType::Root => "根",
                };
                println!("{}{}: {:?}", &sta.name, ty, time);
            }
            println!();
        }

        res2
    }
}

#[test]
fn test() {
    let data = weaverail_model::test_data::diagram_root::get_test_data();
    let graph = WeftGraph::new(
        &data.root,
        data.root.timetables.values().find(|_| true).unwrap().id,
    );
    let start = std::time::Instant::now();
    let _weaved: bellman_ford::Paths<NodeIndex, f64> = graph.weave();
    println!("weave: {}ms", start.elapsed().as_millis())
}
