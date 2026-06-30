use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use weaverail_model::{model::{
    DiagramRoot,
    line_segment::LineSegmentId,
    station::StationId,
    template_train::StopType as TemplateTrainStopType,
    time::Time,
    timetable::{Timetable, TimetableId},
    train::{Train, TrainId},
}, result_weft::{LookupNodeKey, NodeId, NodeType, WeftNode, WeftTempObj}};
use weaverail_model::result_weft::StopType;

struct NumberIssuer {
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

pub fn make_node(root: &DiagramRoot, timetable_id: TimetableId) -> WeftTempObj {
    let mut number_issuer = NumberIssuer::new();
    let timetable = root.timetables.get(&timetable_id).unwrap();
    let trains: Vec<&Train> = root
        .trains
        .values()
        .filter(|train| train.timetable_id == timetable_id)
        .collect();

    let mut result = WeftTempObj {
        nodes: Vec::with_capacity(trains.len() * 100),
        lookup: FxHashMap::default(),
    };

    // add root node
    {
        let root_node = WeftNode {
            node_id: number_issuer.next(),
            station_id: StationId::default(),
            train_id: TrainId::default(),
            segment_id: LineSegmentId::default(),
            edges: SmallVec::new(),
            node_type: NodeType::Root,
            stop_type: StopType::Pass,
        };
        result.nodes.push(root_node);
    }

    // add node
    for train in trains {
        add_train_node(root, train, &mut number_issuer, &mut result);
    }

    result.lookup.shrink_to(result.nodes.len());
    for i in 0..result.nodes.len() {
        let node = &result.nodes[i];
        result.lookup.insert(
            LookupNodeKey::new(node.train_id, node.segment_id, node.node_type),
            i,
        );
    }
    connect_hatsuhatsu_edge(timetable, &mut result);
    connect_chakuchaku_edge(timetable, &mut result);

    result
}

/// 1つの列車の時刻ノードを生成する関数
fn add_train_node(
    diagram_root: &DiagramRoot,
    train: &Train,
    issuer: &mut NumberIssuer,
    tmp_obj: &mut WeftTempObj,
) {
    let mut before_node_index: usize = 0;
    let root_node_id: NodeId = tmp_obj.nodes.get(before_node_index).unwrap().node_id;

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
                stop_type: match start.stop_time {
                    TemplateTrainStopType::Stop(_) => StopType::Stop,
                    TemplateTrainStopType::Pass => StopType::Pass,
                },
            };
            match start.stop_time {
                weaverail_model::model::template_train::StopType::Stop(time) => {
                    if tmp_obj.nodes.get(before_node_index).unwrap().node_id == root_node_id {
                        tmp_obj.nodes[before_node_index]
                            .edges
                            .push((departure_node.node_id, train.start_departure_time))
                    } else {
                        tmp_obj.nodes[before_node_index]
                            .edges
                            .push((departure_node.node_id, time))
                    }
                }
                weaverail_model::model::template_train::StopType::Pass => tmp_obj.nodes
                    [before_node_index]
                    .edges
                    .push((departure_node.node_id, Time::new(0, 0, 0))),
            }
            tmp_obj.nodes.push(departure_node);
            before_node_index = tmp_obj.nodes.len() - 1;

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
            tmp_obj.nodes[before_node_index]
                .edges
                .push((arrival_node.node_id, template_segment.running_time));

            tmp_obj.nodes.push(arrival_node);
            before_node_index = tmp_obj.nodes.len() - 1;
        }
    }
}

fn connect_hatsuhatsu_edge(timetable: &Timetable, tmp_obj: &mut WeftTempObj) {
    for orders in timetable.segment_train_orders.values() {
        for train_ids in orders.order.windows(2) {
            let before_tid = train_ids[0];
            let current_tid = train_ids[1];
            let before_node_index = *tmp_obj
                .lookup
                .get(&LookupNodeKey::new(
                    before_tid,
                    orders.segment_id,
                    NodeType::Departure,
                ))
                .unwrap();
            let current_node_index = *tmp_obj
                .lookup
                .get(&LookupNodeKey::new(
                    current_tid,
                    orders.segment_id,
                    NodeType::Departure,
                ))
                .unwrap();
            let current_node_id = tmp_obj.nodes.get(current_node_index).unwrap().node_id;
            tmp_obj.nodes[before_node_index]
                .edges
                .push((current_node_id, Time::new(0, 2, 0)));
        }
    }
}

fn connect_chakuchaku_edge(timetable: &Timetable, tmp_obj: &mut WeftTempObj) {
    for orders in timetable.segment_train_orders.values() {
        for train_ids in orders.order.windows(2) {
            let before_tid = train_ids[0];
            let current_tid = train_ids[1];
            let before_node_index = *tmp_obj
                .lookup
                .get(&LookupNodeKey::new(
                    before_tid,
                    orders.segment_id,
                    NodeType::Arrival,
                ))
                .unwrap();
            let current_node_index = *tmp_obj
                .lookup
                .get(&LookupNodeKey::new(
                    current_tid,
                    orders.segment_id,
                    NodeType::Arrival,
                ))
                .unwrap();
            let current_node_id = tmp_obj.nodes.get(current_node_index).unwrap().node_id;
            tmp_obj.nodes[before_node_index]
                .edges
                .push((current_node_id, Time::new(0, 2, 0)));
        }
    }
}
