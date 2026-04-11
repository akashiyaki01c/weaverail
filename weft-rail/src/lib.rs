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
    /// 駅間が逆転しているか
    pub is_reversed: bool,
    /// 駅ID
    pub station_id: Uuid,
    /// ノードの種類
    pub node_type: GraphNodeType,
}
impl GraphNodeId {
    pub fn new(
        train_id: Uuid,
        segment_id: Uuid,
        is_reversed: bool,
        station_id: Uuid,
        node_type: GraphNodeType,
    ) -> Self {
        Self {
            train_id,
            segment_id,
            is_reversed,
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

    /// 対象となる時刻表を取得する
    fn timetable(&self) -> &Timetable {
        self.root
            .timetables
            .get(&self.timetable_id)
            .expect("timetables error")
    }

    /// 素の列車ノード/エッジを作成する
    fn make_train_node(
        &'a self,
        graph1: &mut petgraph::Graph<GraphNodeId, f64>,
        nodes: &mut HashMap<GraphNodeId, NodeIndex>,
        train_wrappers: &mut Vec<TrainWrapper<'a>>,
        root_node: &NodeIndex,
    ) {
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
                        segment.segment_id,
                        segment.is_reversed,
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
                            *root_node,
                            start_node_index,
                            Self::conv_time(train.start_departure_time),
                        );
                    } else {
                        let last = train_wrapper.nodes.last().unwrap();
                        match start.stop_time {
                            StopType::Pass => graph1.add_edge(last.1, start_node_index, 0.0),
                            StopType::Stop(time) => {
                                graph1.add_edge(last.1, start_node_index, Self::conv_time(time))
                            }
                        };
                    }
                    train_wrapper
                        .nodes
                        .push((start_node.clone(), start_node_index));

                    let end_node = GraphNodeId::new(
                        train.id,
                        segment.segment_id,
                        segment.is_reversed,
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
                        graph1.add_edge(
                            last.1,
                            end_node_index,
                            Self::conv_time(segment.running_time),
                        );
                    }
                    train_wrapper.nodes.push((end_node.clone(), end_node_index));
                }
            }
            train_wrappers.push(train_wrapper);
        }
    }

    /// 時刻からグラフで使用する表現への変換
    fn conv_time(time: Time) -> f64 {
        -Into::<f64>::into(time.total_second())
    }
    /// グラフで使用する表現から時刻への変換
    fn to_time(time: f64) -> Time {
        Time::new_from_total_second(-time as u32)
    }

    /// 待避情報エッジの追加
    fn add_waiting_edge(
        &self,
        train_wrappers: &mut Vec<TrainWrapper>,
        graph1: &mut petgraph::Graph<GraphNodeId, f64>,
    ) {
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
                        Self::conv_time(Time::new(0, 2, 0)),
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
                        Self::conv_time(Time::new(0, 2, 0)),
                    );
                }
            }
        }
    }

    /// SegmentInfoの構築
    fn get_segment_info(
        &self,
        train_wrappers: &Vec<TrainWrapper>,
        res1: &bellman_ford::Paths<NodeIndex, f64>,
    ) -> Vec<SegmentInfo> {
        let mut segment_infos: Vec<SegmentInfo> = Vec::new();
        for segments in self.root.lines.values().flat_map(|line| &line.segments) {
            segment_infos.push(SegmentInfo {
                segment_id: segments.id,
                train_times_prograde: HashMap::new(),
                train_times_retrograde: HashMap::new(),
            });
        }
        for train in train_wrappers {
            for template_segment in &train.train.template_segments {
                let template_train = self
                    .root
                    .template_trains
                    .get(&template_segment.template_train_id)
                    .expect("error");
                for (_start, segment, _end) in template_train.get_filtered_segment_iter(
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
                            node.0.segment_id == segment.segment_id
                                && node.0.node_type == GraphNodeType::DepartureTime
                        })
                        .unwrap();
                    let arrival_node_id = train
                        .nodes
                        .iter()
                        .find(|node| {
                            node.0.segment_id == segment.segment_id
                                && node.0.node_type == GraphNodeType::ArrivalTime
                        })
                        .unwrap();
                    println!(
                        "dep:{} arr:{}",
                        res1.distances[departure_node_id.1.index()],
                        res1.distances[arrival_node_id.1.index()]
                    );
                    let departure_time = Self::to_time(res1.distances[departure_node_id.1.index()]);
                    let arrival_time = Self::to_time(res1.distances[arrival_node_id.1.index()]);
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
        segment_infos
    }

    pub fn weave(&self) -> bellman_ford::Paths<NodeIndex, f64> {
        let mut graph1: petgraph::Graph<GraphNodeId, f64> = DiGraph::<GraphNodeId, f64>::new();
        let mut nodes: HashMap<GraphNodeId, NodeIndex> = HashMap::new();

        // 根ノード
        let root_node = {
            let node_id = GraphNodeId::new(
                Uuid::nil(),
                Uuid::nil(),
                false,
                Uuid::nil(),
                GraphNodeType::Root,
            );
            let index = graph1.add_node(node_id.clone());
            nodes.insert(node_id.clone(), index);
            index
        };

        let mut train_wrappers = Vec::new();
        self.make_train_node(&mut graph1, &mut nodes, &mut train_wrappers, &root_node);
        self.add_waiting_edge(&mut train_wrappers, &mut graph1);

        let res1: bellman_ford::Paths<NodeIndex, f64> = bellman_ford(&graph1, root_node).unwrap();

        let mut graph2 = graph1.clone();
        let segment_infos: Vec<SegmentInfo> = self.get_segment_info(&train_wrappers, &res1);

        for train_wrapper in &train_wrappers {
            let mut current_before_train_info: Option<Uuid> = None;
            for node in &train_wrapper.nodes {
                println!(
                    "\n{}発列車 [{}]",
                    train_wrapper.train.start_departure_time,
                    self.root.stations.get(&node.0.station_id).unwrap().name,
                );
                if node.0.node_type == GraphNodeType::DepartureTime {
                    // 発発間隔照査
                    let current_segment_id = node.0.segment_id;
                    let current_segment_index = train_wrapper
                        .nodes
                        .iter()
                        .position(|node| {
                            node.0.segment_id == current_segment_id
                                && node.0.node_type == GraphNodeType::DepartureTime
                        })
                        .unwrap();
                    if current_segment_index == 0 {
                        // 前区間情報なし
                        let current_info = segment_infos
                            .iter()
                            .find(|info| info.segment_id == node.0.segment_id)
                            .unwrap();
                        let mut current_trains: Vec<(&Uuid, &(Time, Time, Uuid))> =
                            if node.0.is_reversed {
                                &current_info.train_times_retrograde
                            } else {
                                &current_info.train_times_prograde
                            }
                            .iter()
                            .collect();
                        current_trains.sort_by_key(|&(_uuid, times)| times.0);
                        println!("{:?}", current_trains);

                        let current_train_index = current_trains
                            .iter()
                            .position(|train| *train.0 == train_wrapper.train.id)
                            .unwrap();
                        if current_train_index != 0 {
                            // 前列車存在
                            let before_train = current_trains.get(current_train_index - 1).unwrap();
                            current_before_train_info = Some(*before_train.0);
                            println!(
                                "[{}] 変更 **前区間情報なし, 前列車存在** 現在列車:{} 前列車:{}発",
                                self.root.stations.get(&node.0.station_id).unwrap().name,
                                train_wrapper.train.start_departure_time,
                                self.timetable()
                                    .trains
                                    .values()
                                    .find(|train| train.id == *before_train.0)
                                    .unwrap()
                                    .start_departure_time
                            );
                        } else {
                            current_before_train_info = None;
                            println!(
                                "[{}] 削除 **前区間情報なし, 前列車存在なし**",
                                self.root.stations.get(&node.0.station_id).unwrap().name,
                            );
                        }
                    } else {
                        // 前区間情報あり
                        let current_info = segment_infos
                            .iter()
                            .find(|info| info.segment_id == node.0.segment_id)
                            .unwrap();
                        // 現区間での列車順序
                        let current_trains: Vec<(&Uuid, &(Time, Time, Uuid))> = {
                            let mut trains: Vec<(&Uuid, &(Time, Time, Uuid))> =
                                if node.0.is_reversed {
                                    &current_info.train_times_retrograde
                                } else {
                                    &current_info.train_times_prograde
                                }
                                .iter()
                                .collect();
                            trains.sort_by_key(|&(_uuid, times)| times.0);
                            trains
                        };

                        // 現区間での現列車の列車順序index
                        let current_current_train_index = current_trains
                            .iter()
                            .position(|train| *train.0 == train_wrapper.train.id)
                            .unwrap();
                        if current_current_train_index != 0 {
                            // 前列車存在
                            // 現区間での前列車の列車順序index
                            let current_before_train_index = current_current_train_index - 1;
                            // 前区間情報
                            let before_segment =
                                train_wrapper.nodes.get(current_segment_index - 1).unwrap();
                            let before_info = segment_infos
                                .iter()
                                .find(|info| info.segment_id == before_segment.0.segment_id)
                                .unwrap();
                            // 前区間での列車順序
                            let before_trains: Vec<(&Uuid, &(Time, Time, Uuid))> = {
                                let mut trains: Vec<(&Uuid, &(Time, Time, Uuid))> =
                                    if node.0.is_reversed {
                                        &before_info.train_times_retrograde
                                    } else {
                                        &before_info.train_times_prograde
                                    }
                                    .iter()
                                    .collect();
                                trains.sort_by_key(|&(_uuid, times)| times.0);
                                trains
                            };
                            // 前区間での現列車の列車順序index
                            let before_current_train_index = before_trains
                                .iter()
                                .position(|train| *train.0 == train_wrapper.train.id)
                                .unwrap();
                            // 前区間での前列車の列車順序index
                            if before_current_train_index != 0 {
                                // 前区間に前列車が存在している
                                let before_before_train_inedx = before_current_train_index - 1;
                                let current_before_train_id =
                                    current_trains.get(current_before_train_index).unwrap().0;
                                let before_before_train_id =
                                    before_trains.get(before_before_train_inedx).unwrap().0;
                                if current_before_train_id != before_before_train_id {
                                    // 現区間と前区間で先行列車が異なる
                                    // 待避がある場合はescape
                                    let adjustment =
                                        self.timetable().adjustments.values().find(|adjustment| {
                                            match adjustment.adjustment {
                                                TrainsAdjustmentType::None => false,
                                                TrainsAdjustmentType::Waiting {
                                                    station_id,
                                                    waiting_track_id: _,
                                                    waiting_train_id,
                                                    passing_train_id,
                                                } => {
                                                    println!(
                                                        "{:?}; sta:{} before:{} current:{}",
                                                        adjustment.adjustment,
                                                        node.0.station_id,
                                                        current_before_train_id,
                                                        node.0.train_id
                                                    );
                                                    if station_id == node.0.station_id {
                                                        if waiting_train_id != node.0.train_id
                                                            && passing_train_id != node.0.train_id
                                                        {
                                                            false
                                                        } else {
                                                            true
                                                        }
                                                    } else {
                                                        println!("駅不合");
                                                        false
                                                    }
                                                }
                                            }
                                        });
                                    if adjustment.is_some() {
                                        current_before_train_info = None;
                                        println!(
                                            "[{}]　削除 **前区間情報あり, 前列車あり, 前区間に同一前列車なし, 待避あり**",
                                            self.root
                                                .stations
                                                .get(&node.0.station_id)
                                                .unwrap()
                                                .name,
                                        );
                                    } else {
                                        if current_before_train_info.is_none() {
                                            let before_train = current_trains
                                                .get(current_before_train_index)
                                                .unwrap();
                                            current_before_train_info = Some(*before_train.0);
                                        }
                                        println!(
                                            "[{}] 継続 **前区間情報あり, 前列車あり, 前区間に同一前列車なし, 待避なし**",
                                            self.root
                                                .stations
                                                .get(&node.0.station_id)
                                                .unwrap()
                                                .name,
                                        );
                                    }
                                } else {
                                    // 現区間と前区間で先行列車が同一
                                    // 現区間と前区間で先行列車が異なる
                                    // 待避がある場合はescape
                                    let adjustment =
                                        self.timetable().adjustments.values().find(|adjustment| {
                                            match adjustment.adjustment {
                                                TrainsAdjustmentType::None => false,
                                                TrainsAdjustmentType::Waiting {
                                                    station_id,
                                                    waiting_track_id: _,
                                                    waiting_train_id,
                                                    passing_train_id,
                                                } => {
                                                    println!(
                                                        "{:?}; sta:{} before:{} current:{}",
                                                        adjustment.adjustment,
                                                        node.0.station_id,
                                                        current_before_train_id,
                                                        node.0.train_id
                                                    );
                                                    if station_id == node.0.station_id {
                                                        if waiting_train_id != node.0.train_id
                                                            && passing_train_id != node.0.train_id
                                                        {
                                                            false
                                                        } else {
                                                            true
                                                        }
                                                    } else {
                                                        println!("駅不合");
                                                        false
                                                    }
                                                }
                                            }
                                        });
                                    if adjustment.is_some() {
                                        current_before_train_info = None;
                                        println!(
                                            "[{}]　削除 **前区間情報あり, 前列車あり, 前区間に同一前列車あり, 待避あり**",
                                            self.root
                                                .stations
                                                .get(&node.0.station_id)
                                                .unwrap()
                                                .name,
                                        );
                                    } else {
                                        if current_before_train_info.is_none() {
                                            let before_train = current_trains
                                                .get(current_before_train_index)
                                                .unwrap();
                                            current_before_train_info = Some(*before_train.0);
                                        }
                                        println!(
                                            "[{}] 継続 **前区間情報あり, 前列車あり, 前区間に同一前列車あり, 待避なし**",
                                            self.root
                                                .stations
                                                .get(&node.0.station_id)
                                                .unwrap()
                                                .name,
                                        );
                                    }
                                }
                            } else {
                                // 前区間に前列車となる列車が存在しない
                                current_before_train_info = None;
                                println!(
                                    "[{}] 削除 **前区間情報あり, 前列車あり, 前区間に前列車なし**",
                                    self.root.stations.get(&node.0.station_id).unwrap().name,
                                );
                            }
                        } else {
                            current_before_train_info = None;
                            println!(
                                "[{}] 削除 **前区間情報あり, 前列車なし**",
                                self.root.stations.get(&node.0.station_id).unwrap().name,
                            );
                        }
                    }

                    // ノード
                    if let Some(current_before_train_info) = current_before_train_info {
                        let before_train_wrapper = train_wrappers
                            .iter()
                            .find(|wrapper| wrapper.train.id == current_before_train_info)
                            .unwrap();
                        let before_node = before_train_wrapper
                            .nodes
                            .iter()
                            .find(|v| {
                                v.0.station_id == node.0.station_id
                                    && v.0.node_type == node.0.node_type
                            })
                            .unwrap();
                        println!(
                            "[{}] {}発列車→{}発列車",
                            self.root
                                .stations
                                .get(
                                    &train_wrapper
                                        .nodes
                                        .get(current_segment_index)
                                        .unwrap()
                                        .0
                                        .station_id
                                )
                                .unwrap()
                                .name,
                            before_train_wrapper.train.start_departure_time,
                            train_wrapper.train.start_departure_time
                        );
                        graph2.add_edge(before_node.1, node.1, Self::conv_time(Time::new(0, 2, 0)));
                    }
                } else if node.0.node_type == GraphNodeType::ArrivalTime {
                    // 着着間隔照査
                }
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
