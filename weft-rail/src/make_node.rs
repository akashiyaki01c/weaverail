use std::collections::HashMap;

use smallvec::SmallVec;
use weaverail_model::model::{
    DiagramRoot,
    line::LineSegmentId,
    station::StationId,
    time::Time,
    timetable::{Timetable, TimetableId},
    train::{Train, TrainId},
};

use crate::{NodeId, NodeType, WeftNode};

struct NumberIssuer {
    current: usize,
}
impl NumberIssuer {
    pub fn new() -> Self {
        Self { current: 0 }
    }

    pub fn next(&mut self) -> NodeId {
        self.current += 1;
        NodeId(self.current)
    }
}

/// ノードを生成する関数
pub fn make_node(root: &DiagramRoot, timetable_id: TimetableId) -> HashMap<NodeId, WeftNode> {
    let mut number_issuer = NumberIssuer::new();
    let timetable = root.timetables.get(&timetable_id).unwrap();
    let mut result: HashMap<TrainId, Vec<WeftNode>> = HashMap::new();
    let mut root_node = WeftNode {
        node_id: number_issuer.next(),
        station_id: StationId::new(),
        train_id: TrainId::new(),
        segment_id: LineSegmentId::new(),
        edges: SmallVec::new(),
        node_type: NodeType::Root,
    };

    // 列車の生時刻の取得
    for train in timetable.trains.values() {
        let nodes = make_train_node(root, train, &mut root_node, &mut number_issuer);
        result.insert(train.id, nodes);
    }

	connect_hatsuhatsu_edge(root, timetable, &mut result);
	connect_chakuchaku_edge(root, timetable, &mut result);

    result.into_values().flatten().map(|node| (node.node_id, node)).collect()
}

/// 1つの列車時刻のノードを生成する関数
fn make_train_node(
    diagram_root: &DiagramRoot,
    train: &Train,
    root_node: &mut WeftNode,
    issuer: &mut NumberIssuer,
) -> Vec<WeftNode> {
    let mut result = Vec::new();
    let mut before_node: &mut WeftNode = root_node;
    for template_segment in &train.template_segments {
        let template_train = diagram_root
            .template_trains
            .get(&template_segment.template_train_id)
            .unwrap();
        for (start, template_segment, end) in template_train.get_filtered_segment_iter(
            template_segment.start_station_id,
            template_segment.end_station_id,
        ) {
            let departure_node = WeftNode {
                node_id: issuer.next(),
                station_id: start.station_id,
                train_id: train.id,
                segment_id: template_segment.segment_id,
                edges: SmallVec::new(),
                node_type: NodeType::Departure,
            };
            match start.stop_time {
                weaverail_model::model::template_train::StopType::Stop(time) => {
                    before_node.edges.push((departure_node.node_id, time))
                }
                weaverail_model::model::template_train::StopType::Pass => before_node
                    .edges
                    .push((departure_node.node_id, Time::new(0, 0, 0))),
            }
            result.push(departure_node);
            before_node = result.last_mut().unwrap();

            let arrival_node = WeftNode {
                node_id: issuer.next(),
                station_id: end.station_id,
                train_id: train.id,
                segment_id: template_segment.segment_id,
                edges: SmallVec::new(),
                node_type: NodeType::Arrival,
            };
            before_node
                .edges
                .push((arrival_node.node_id, template_segment.running_time));
            result.push(arrival_node);
            before_node = result.last_mut().unwrap();
        }
    }
    result
}

/// 発発時隔エッジを追加する関数
fn connect_hatsuhatsu_edge(
    diagram_root: &DiagramRoot,
    timetable: &Timetable,
    nodes: &mut HashMap<TrainId, Vec<WeftNode>>,
) {
    for orders in timetable.segment_train_orders.values() {
        let line_segment = diagram_root.get_segment(orders.segment_id).unwrap();
        let segment_start_id = line_segment.start_station;
        for train_ids in orders.order.windows(2) {
            let before = train_ids[0];
            let current = train_ids[1];

			let current_station_node = {
                let current_train_nodes = nodes.get(&current).unwrap();
                current_train_nodes
                    .iter()
                    .find(|node| {
                        node.station_id == segment_start_id
                            && node.segment_id == orders.segment_id
                            && node.node_type == NodeType::Departure
                    })
                    .unwrap().node_id
            };

            let before_station_nodes = {
                let before_train_nodes = nodes.get_mut(&before).unwrap();
                before_train_nodes
                    .iter_mut()
                    .find(|node| {
                        node.station_id == segment_start_id
                            && node.segment_id == orders.segment_id
                            && node.node_type == NodeType::Departure
                    })
                    .unwrap()
            };

            before_station_nodes
                .edges
                .push((current_station_node, Time::new(0, 2, 0)));
        }
    }
}

/// 着着時隔エッジを追加する関数
fn connect_chakuchaku_edge(
    diagram_root: &DiagramRoot,
    timetable: &Timetable,
    nodes: &mut HashMap<TrainId, Vec<WeftNode>>,
) {
    for orders in timetable.segment_train_orders.values() {
        let line_segment = diagram_root.get_segment(orders.segment_id).unwrap();
        let segment_end_id = line_segment.end_station;
        for train_ids in orders.order.windows(2) {
            let before = train_ids[0];
            let current = train_ids[1];

			let current_station_node = {
                let current_train_nodes = nodes.get(&current).unwrap();
                current_train_nodes
                    .iter()
                    .find(|node| {
                        node.station_id == segment_end_id
                            && node.segment_id == orders.segment_id
                            && node.node_type == NodeType::Arrival
                    })
                    .unwrap().node_id
            };

            let before_station_nodes = {
                let before_train_nodes = nodes.get_mut(&before).unwrap();
                before_train_nodes
                    .iter_mut()
                    .find(|node| {
                        node.station_id == segment_end_id
                            && node.segment_id == orders.segment_id
                            && node.node_type == NodeType::Arrival
                    })
                    .unwrap()
            };

            before_station_nodes
                .edges
                .push((current_station_node, Time::new(0, 2, 0)));
        }
    }
}