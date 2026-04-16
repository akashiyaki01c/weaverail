use std::collections::HashMap;

use weaverail_model::model::{
    line::LineSegmentId, time::Time, timetable::Timetable, train::TrainId,
};

use crate::{NodeType, WeftNode};

pub fn update_node(
    timetable: &Timetable,
    nodes: &mut (WeftNode, HashMap<TrainId, Vec<WeftNode>>),
    update_type: UpdateType,
) {
    match update_type {
        UpdateType::ChangeStartStationDepartureTime(_, _) => {
            // no-op
        }
        UpdateType::ChangeTrainOrder(change_orders) => {
            for (change_segment_id, before_train_id, current_train_id) in change_orders {
                let change_segment = timetable
                    .segment_train_orders
                    .get(&change_segment_id)
                    .unwrap();
                let before_train_index = change_segment
                    .order
                    .iter()
                    .position(|train| *train == before_train_id)
                    .unwrap();
                let current_train_index = change_segment
                    .order
                    .iter()
                    .position(|train| *train == current_train_id)
                    .unwrap();
                if current_train_index - before_train_index != 1 {
                    // no-op
                    unreachable!()
                }
                if before_train_index == 0 {
                    // 前列車が存在しない
                    // 現列車の(前列車→現列車)のエッジを削除
                    // 前列車に(現列車→前列車)のエッジを追加

                    {
                        let current_train_node_id = nodes
                            .1
                            .get(&current_train_id)
                            .unwrap()
                            .iter()
                            .find(|node| {
                                node.segment_id == change_segment_id
                                    && node.node_type == NodeType::Departure
                            })
                            .unwrap()
                            .node_id;
                        let before_train_node: &mut WeftNode = nodes
                            .1
                            .get_mut(&before_train_id)
                            .unwrap()
                            .iter_mut()
                            .find(|node| {
                                node.segment_id == change_segment_id
                                    && node.node_type == NodeType::Departure
                            })
                            .unwrap();
                        let index = before_train_node
                            .edges
                            .iter()
                            .position(|edge| edge.0 == current_train_node_id)
                            .unwrap();
                        before_train_node.edges.remove(index);
                    }
                    {
                        let before_train_node_id = nodes
                            .1
                            .get(&before_train_id)
                            .unwrap()
                            .iter()
                            .find(|node| {
                                node.segment_id == change_segment_id
                                    && node.node_type == NodeType::Departure
                            })
                            .unwrap()
                            .node_id;
                        let current_train_node: &mut WeftNode = nodes
                            .1
                            .get_mut(&current_train_id)
                            .unwrap()
                            .iter_mut()
                            .find(|node| {
                                node.segment_id == change_segment_id
                                    && node.node_type == NodeType::Departure
                            })
                            .unwrap();
                        current_train_node
                            .edges
                            .push((before_train_node_id, Time::new(0, 2, 0)));
                    }
                    {
                        let current_train_node_id = nodes
                            .1
                            .get(&current_train_id)
                            .unwrap()
                            .iter()
                            .find(|node| {
                                node.segment_id == change_segment_id
                                    && node.node_type == NodeType::Arrival
                            })
                            .unwrap()
                            .node_id;
                        let before_train_node: &mut WeftNode = nodes
                            .1
                            .get_mut(&before_train_id)
                            .unwrap()
                            .iter_mut()
                            .find(|node| {
                                node.segment_id == change_segment_id
                                    && node.node_type == NodeType::Arrival
                            })
                            .unwrap();
                        let index = before_train_node
                            .edges
                            .iter()
                            .position(|edge| edge.0 == current_train_node_id)
                            .unwrap();
                        before_train_node.edges.remove(index);
                    }
                    {
                        let before_train_node_id = nodes
                            .1
                            .get(&before_train_id)
                            .unwrap()
                            .iter()
                            .find(|node| {
                                node.segment_id == change_segment_id
                                    && node.node_type == NodeType::Arrival
                            })
                            .unwrap()
                            .node_id;
                        let current_train_node: &mut WeftNode = nodes
                            .1
                            .get_mut(&current_train_id)
                            .unwrap()
                            .iter_mut()
                            .find(|node| {
                                node.segment_id == change_segment_id
                                    && node.node_type == NodeType::Arrival
                            })
                            .unwrap();
                        current_train_node
                            .edges
                            .push((before_train_node_id, Time::new(0, 2, 0)));
                    }
                }
            }
        }
    }
}

/// 更新の種類
pub enum UpdateType {
    /// 始発駅の時刻変更
    ChangeStartStationDepartureTime(TrainId, Time),
    /// 列車の順序変更
    ChangeTrainOrder(Vec<(LineSegmentId, TrainId, TrainId)>),
}
