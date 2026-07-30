//! # make_node
//!
//! 列車時刻ノードを生成し、有向グラフを構築するモジュール
//!
//! グラフのデータ定義は、`weverail_model::result_weft`モジュールにある

use std::{collections::HashMap, sync::LazyLock};

use smallvec::SmallVec;
use weaverail_model::{
    error::ModelError,
    model::{
        DiagramRoot,
        line_segment::LineSegmentId,
        station::StationId,
        template_train::StopType as TemplateTrainStopType,
        time::Time,
        timetable::{Timetable, TimetableId},
        train::{Train, TrainId},
    },
    result_weft::{NodeId, NodeType, StopType, WeftNode},
};

type NodeKey = (TrainId, LineSegmentId, NodeType);

/// NULL値を表す
static DUMMY_NODE: LazyLock<WeftNode> = LazyLock::new(WeftNode::default);

/// 一意のIDを生成する簡易的な構造体
struct NumberIssuer {
    /// 現在発行済の最大番号
    current: usize,
}
impl NumberIssuer {
    pub fn new() -> Self {
        Self { current: 0 }
    }

    pub fn next(&mut self) -> NodeId {
        let result = NodeId::new(self.current);
        self.current += 1;
        result
    }
}

/// 有向グラフのノードを生成する関数
pub fn make_node(
    root: &DiagramRoot,
    timetable_id: TimetableId,
) -> Result<(WeftNode, HashMap<TrainId, Vec<WeftNode>>), ModelError> {
    let mut number_issuer = NumberIssuer::new();
    let timetable = root
        .timetables
        .get(&timetable_id)
        .ok_or(ModelError::ObjectNotFound)?;
    let trains: Vec<&Train> = root
        .trains
        .values()
        .filter(|train| train.timetable_id == timetable_id)
        .collect();
    let mut result: HashMap<TrainId, Vec<WeftNode>> = HashMap::with_capacity(trains.len());
    let mut root_node = WeftNode {
        node_id: number_issuer.next(),
        station_id: StationId::default(),
        train_id: TrainId::default(),
        segment_id: LineSegmentId::default(),
        edges: SmallVec::new(),
        node_type: NodeType::Root,
        stop_type: StopType::Pass,
    };

    // 列車の生時刻の取得
    for train in root.trains.values().filter(|train| trains.contains(train)) {
        let nodes = make_train_node(root, train, &mut root_node, &mut number_issuer)?;
        result.insert(train.id, nodes);
    }

    let mut node_lookup: HashMap<NodeKey, NodeId> =
        HashMap::with_capacity(result.values().flatten().count());
    for train_nodes in result.values() {
        for node in train_nodes {
            node_lookup.insert(
                (node.train_id, node.segment_id, node.node_type),
                node.node_id,
            );
        }
    }

    connect_hatsuhatsu_edge(timetable, &mut result, &node_lookup)?;
    connect_chakuchaku_edge(timetable, &mut result, &node_lookup)?;

    Ok((root_node, result))
}

/// NodeIdからノードを取得する関数
/// 計算オーダは `O(nodes.count)`
pub fn get_node_by_nodeid<'a>(
    root_node: &'a WeftNode,
    nodes: &'a HashMap<TrainId, Vec<WeftNode>>,
) -> Vec<&'a WeftNode> {
    let len = nodes.values().flatten().count(); // O(N)
    let mut result: Vec<&WeftNode> = vec![&DUMMY_NODE; len + 1]; // O(N)

    for node in nodes.values().flatten() {
        // O(N)
        result[node.node_id.0] = node; // O(1)
    }
    result[root_node.node_id.0] = root_node;

    result
}

/// 1つの列車時刻のノードを生成する関数
/// 計算オーダは `O(segment.length)`
fn make_train_node(
    diagram_root: &DiagramRoot,
    train: &Train,
    root_node: &mut WeftNode,
    issuer: &mut NumberIssuer,
) -> Result<Vec<WeftNode>, ModelError> {
    let mut result = Vec::new();
    let root_node_id = root_node.node_id;
    let mut before_node: &mut WeftNode = root_node;
    for template_segment in &train.template_segments {
        let template_train = diagram_root
            .template_trains
            .get(&template_segment.template_train_id)
            .ok_or(ModelError::ObjectNotFound)?;
        for (start, template_segment, end) in template_train.get_filtered_segment_iter(
            template_segment.start_station_id,
            template_segment.end_station_id,
        )? {
            let departure_node = WeftNode {
                node_id: issuer.next(),
                station_id: start.station_id,
                train_id: train.id,
                segment_id: template_segment.segment_id,
                edges: SmallVec::new(),
                node_type: NodeType::Departure,
                stop_type: match start.stop_time {
                    TemplateTrainStopType::Stop(_) => StopType::Stop,
                    TemplateTrainStopType::Pass => StopType::Pass,
                },
            };
            match start.stop_time {
                weaverail_model::model::template_train::StopType::Stop(time) => {
                    if before_node.node_id == root_node_id {
                        before_node
                            .edges
                            .push((departure_node.node_id, train.start_departure_time))
                    } else {
                        before_node.edges.push((departure_node.node_id, time))
                    }
                }
                weaverail_model::model::template_train::StopType::Pass => before_node
                    .edges
                    .push((departure_node.node_id, Time::new(0, 0, 0))),
            }
            result.push(departure_node);
            before_node = result.last_mut().ok_or(ModelError::ObjectNotFound)?;

            let arrival_node = WeftNode {
                node_id: issuer.next(),
                station_id: end.station_id,
                train_id: train.id,
                segment_id: template_segment.segment_id,
                edges: SmallVec::new(),
                node_type: NodeType::Arrival,
                stop_type: match end.stop_time {
                    TemplateTrainStopType::Stop(_) => StopType::Stop,
                    TemplateTrainStopType::Pass => StopType::Pass,
                },
            };
            before_node
                .edges
                .push((arrival_node.node_id, template_segment.running_time));
            result.push(arrival_node);
            before_node = result.last_mut().ok_or(ModelError::ObjectNotFound)?;
        }
    }
    Ok(result)
}

/// 発発時隔エッジを追加する関数
/// 計算オーダは `O(segment_train_orders.length * train-time-node.length)`
fn connect_hatsuhatsu_edge(
    timetable: &Timetable,
    nodes: &mut HashMap<TrainId, Vec<WeftNode>>,
    node_lookup: &HashMap<NodeKey, NodeId>,
) -> Result<(), ModelError> {
    for orders in timetable.segment_train_orders.values() {
        // 順行列車
        for train_ids in orders.prograde.order.windows(2) {
            let before_tid = train_ids[0];
            let current_tid = train_ids[1];

            let current_node_id = *node_lookup
                .get(&(current_tid, orders.prograde.segment_id, NodeType::Departure))
                .ok_or(ModelError::ObjectNotFound)?;

            let before_train_nodes = nodes
                .get_mut(&before_tid)
                .ok_or(ModelError::ObjectNotFound)?;

            let before_node_id = *node_lookup
                .get(&(before_tid, orders.prograde.segment_id, NodeType::Departure))
                .ok_or(ModelError::ObjectNotFound)?;

            if let Some(node) = before_train_nodes
                .iter_mut()
                .find(|n| n.node_id == before_node_id)
            {
                node.edges.push((current_node_id, Time::new(0, 2, 0)));
            }
        }
        // 逆行列車
        for train_ids in orders.retrograde.order.windows(2) {
            let before_tid = train_ids[0];
            let current_tid = train_ids[1];

            let current_node_id = *node_lookup
                .get(&(current_tid, orders.retrograde.segment_id, NodeType::Departure))
                .ok_or(ModelError::ObjectNotFound)?;

            let before_train_nodes = nodes
                .get_mut(&before_tid)
                .ok_or(ModelError::ObjectNotFound)?;

            let before_node_id = *node_lookup
                .get(&(before_tid, orders.retrograde.segment_id, NodeType::Departure))
                .ok_or(ModelError::ObjectNotFound)?;

            if let Some(node) = before_train_nodes
                .iter_mut()
                .find(|n| n.node_id == before_node_id)
            {
                node.edges.push((current_node_id, Time::new(0, 2, 0)));
            }
        }
    }

    Ok(())
}

/// 着着時隔エッジを追加する関数
/// 計算オーダは `O(segment_train_orders.length * train-time-node.length)`
fn connect_chakuchaku_edge(
    timetable: &Timetable,
    nodes: &mut HashMap<TrainId, Vec<WeftNode>>,
    node_lookup: &HashMap<NodeKey, NodeId>,
) -> Result<(), ModelError> {
    for orders in timetable.segment_train_orders.values() {
        // 順行列車
        for train_ids in orders.prograde.order.windows(2) {
            let before_tid = train_ids[0];
            let current_tid = train_ids[1];

            let current_node_id = *node_lookup
                .get(&(current_tid, orders.prograde.segment_id, NodeType::Arrival))
                .ok_or(ModelError::ObjectNotFound)?;

            let before_train_nodes = nodes
                .get_mut(&before_tid)
                .ok_or(ModelError::ObjectNotFound)?;

            let before_node_id = *node_lookup
                .get(&(before_tid, orders.prograde.segment_id, NodeType::Arrival))
                .ok_or(ModelError::ObjectNotFound)?;

            if let Some(node) = before_train_nodes
                .iter_mut()
                .find(|n| n.node_id == before_node_id)
            {
                node.edges.push((current_node_id, Time::new(0, 2, 0)));
            }
        }
        // 逆行列車
        for train_ids in orders.retrograde.order.windows(2) {
            let before_tid = train_ids[0];
            let current_tid = train_ids[1];

            let current_node_id = *node_lookup
                .get(&(current_tid, orders.retrograde.segment_id, NodeType::Arrival))
                .ok_or(ModelError::ObjectNotFound)?;

            let before_train_nodes = nodes
                .get_mut(&before_tid)
                .ok_or(ModelError::ObjectNotFound)?;

            let before_node_id = *node_lookup
                .get(&(before_tid, orders.retrograde.segment_id, NodeType::Arrival))
                .ok_or(ModelError::ObjectNotFound)?;

            if let Some(node) = before_train_nodes
                .iter_mut()
                .find(|n| n.node_id == before_node_id)
            {
                node.edges.push((current_node_id, Time::new(0, 2, 0)));
            }
        }
    }

    Ok(())
}
