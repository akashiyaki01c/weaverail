//! 列車順序の同定アルゴリズムを実装するモジュール

use weaverail_model::{
    error::ModelError,
    model::{
        DiagramRoot,
        line_segment::LineSegmentId,
        time::Time,
        timetable::TimetableId,
        train::{TemplateSegment, TrainId},
    },
    result_weft::{NodeType, WeftTempObj},
};

/// 始発駅時点における発車順序を求める関数
pub fn get_starting_order_index(
    root: &DiagramRoot,
    obj: &WeftTempObj,
    times: &[Time],
    segment_id: LineSegmentId,
    is_reversed: bool,
    departure_time: Time,
) -> Result<usize, ModelError> {
    let start_sta_id = if is_reversed {
        root.segments
            .get(&segment_id)
            .ok_or(ModelError::ObjectNotFound)?
            .end_station(root)?
            .id
    } else {
        root.segments
            .get(&segment_id)
            .ok_or(ModelError::ObjectNotFound)?
            .start_station(root)?
            .id
    };
    // 始発駅→次駅を走行する列車の発車時刻一覧
    let mut segment_times: Vec<_> = obj
        .nodes
        .iter()
        .filter(|v| {
            v.segment_id == segment_id
                && v.node_type == NodeType::Departure
                && v.station_id == start_sta_id
        })
        .collect();
    segment_times.sort_by(|a, b| times[a.node_id.0].cmp(&times[b.node_id.0]));

    for (i, time) in segment_times.iter().enumerate() {
        println!("{i}: {:?}", times[time.node_id.0]);
        if times[time.node_id.0] > departure_time {
            return Ok(i);
        }
    }

    Ok(0)
}

/// 次区間における発車順序を求める関数
///
/// 戻り値が`Some(usize)`の場合、順序が定まったことを、
/// `None`の場合は順序が定まらず再計算が必要であることを表す。
pub fn get_next_order_index(
    root: &DiagramRoot,
    timetable_id: TimetableId,
    // 前区間の駅間ID
    before_segment_id: LineSegmentId,
    // 前区間が反転しているか
    before_segment_reversed: bool,
    // 次区間の駅間ID
    next_segment_id: LineSegmentId,
    // 次区間が反転しているか
    next_segment_reversed: bool,
    // 前区間での発車順序
    before_segment_order: usize,
) -> Result<Option<usize>, ModelError> {
    let timetable = root
        .timetables
        .get(&timetable_id)
        .ok_or(ModelError::ObjectNotFound)?;
    let before_orders = timetable
        .segment_train_orders
        .get(&before_segment_id)
        .ok_or(ModelError::ObjectNotFound)?;
    // 前区間における順序
    let before_orders = if before_segment_reversed {
        &before_orders.1
    } else {
        &before_orders.0
    };
    // 前区間の先行列車
    let before_preceding_train = if before_segment_order == 0 {
        None
    } else {
        before_orders.order.get(before_segment_order - 1)
    };
    // 前区間での続行列車
    let before_following_train = before_orders.order.get(before_segment_order + 1);

    let next_orders = timetable
        .segment_train_orders
        .get(&next_segment_id)
        .ok_or(ModelError::ObjectNotFound)?;
    // 次区間における順序
    let next_orders = if next_segment_reversed {
        &next_orders.1
    } else {
        &next_orders.0
    };

    // 次区間での先行列車の順序
    let next_before_preceding_train_index = if let Some(preceding_train) = before_preceding_train {
        next_orders
            .order
            .iter()
            .position(|order| order == preceding_train)
    } else {
        None
    };
    // 次区間での続行列車の順序
    let next_before_following_train_index = if let Some(following_train) = before_following_train {
        next_orders
            .order
            .iter()
            .position(|order| order == following_train)
    } else {
        None
    };

    // 条件分岐
    match (before_preceding_train, before_following_train) {
        // 先行列車・続行列車とも前区間に存在しない
        (None, None) => {
            println!("先行列車・続行列車とも前区間に存在しない");
            Ok(None)
        }
        // 先行列車が前区間に存在せず、続行列車が前区間に存在する
        (None, Some(_before_following_train)) => {
            match next_before_following_train_index {
                Some(next_before_following_train_index) => {
                    // 続行列車が次区間にも存在する
                    if next_before_following_train_index == 0 {
                        println!(
                            "先行列車が前区間に存在せず、続行列車が前区間に存在する, 続行列車が次区間にも存在する, 次区間において、続行列車の前に列車が存在しない場合"
                        );
                        // 次区間において、続行列車の前に列車が存在しない場合
                        Ok(Some(next_before_following_train_index))
                    } else {
                        println!(
                            "先行列車が前区間に存在せず、続行列車が前区間に存在する, 続行列車が次区間にも存在する, 次区間において、続行列車の前に列車が存在する場合"
                        );
                        // 次区間において、続行列車の前に列車が存在する場合
                        Ok(None)
                    }
                }
                None => {
                    println!(
                        "先行列車が前区間に存在せず、続行列車が前区間に存在する, 続行列車が次区間に存在しない場合"
                    );
                    // 続行列車が次区間に存在しない
                    Ok(None)
                }
            }
        }
        // 先行列車が前区間に存在し、続行列車が前区間に存在しない
        (Some(_before_preceding_train), None) => {
            match next_before_preceding_train_index {
                Some(next_before_preceding_train_index) => {
                    // 先行列車が次区間にも存在する
                    if next_before_preceding_train_index == next_orders.order.len() - 1 {
                        println!(
                            "先行列車が前区間に存在し、続行列車が前区間に存在しない, 先行列車が次区間にも存在する, 次区間において、先行列車の後に列車が存在しない場合"
                        );
                        // 次区間において、先行列車の後に列車が存在しない場合
                        Ok(Some(next_before_preceding_train_index + 1))
                    } else {
                        println!(
                            "先行列車が前区間に存在し、続行列車が前区間に存在しない, 先行列車が次区間にも存在する, 次区間において、先行列車の後に列車が存在する場合"
                        );
                        // 次区間において、先行列車の後に列車が存在する場合
                        Ok(None)
                    }
                }
                None => {
                    println!(
                        "先行列車が前区間に存在し、続行列車が前区間に存在しない, 先行列車が次区間に存在しない"
                    );
                    // 先行列車が次区間に存在しない
                    Ok(None)
                }
            }
        }
        // 先行列車・続行列車が前区間に存在する
        (Some(_before_preceding_train), Some(_before_following_train)) => {
            match (
                next_before_preceding_train_index,
                next_before_following_train_index,
            ) {
                // 次区間で先行列車・続行列車が存在する
                (
                    Some(next_before_preceding_train_index),
                    Some(next_before_following_train_index),
                ) => {
                    if next_before_preceding_train_index + 1 == next_before_following_train_index {
                        println!(
                            "先行列車・続行列車が前区間に存在する, 次区間で先行列車・続行列車が存在する, 次区間で先行列車→続行列車の順に連続している"
                        );
                        // 次区間で先行列車→続行列車の順に連続している
                        return Ok(Some(next_before_following_train_index));
                    } else if next_before_following_train_index < next_before_preceding_train_index
                    {
                        println!(
                            "先行列車・続行列車が前区間に存在する, 次区間で先行列車・続行列車が存在する, 次区間で続行列車→先行列車の順に連続している (逆転)"
                        );
                        // 次区間で続行列車→先行列車の順に連続している (逆転)
                        // 続行列車と先行列車の間に列車が存在する場合もこれ
                        return Ok(Some(next_before_following_train_index));
                    }

                    // ここで、次区間で列車は、先行列車、続行列車の順番になっている
                    // 先行列車、続行列車の間には他列車が存在している

                    // 次区間での、先行列車〜続行列車に挟まっている列車
                    let between_trains = &next_orders.order
                        [next_before_preceding_train_index + 1..next_before_following_train_index];
                    let has_unknown_train = between_trains
                        .iter()
                        .any(|tid| !before_orders.order.contains(tid));
                    if has_unknown_train {
                        println!(
                            "先行列車・続行列車が前区間に存在する, 次区間で先行列車・続行列車が存在する, 先行列車、続行列車の間には他列車が存在している, 前区間にない列車がある場合"
                        );
                        // 前区間にない列車がある場合
                        return Ok(None);
                    }

                    println!(
                        "先行列車・続行列車が前区間に存在する, 次区間で先行列車・続行列車が存在する, 先行列車、続行列車の間には他列車が存在している, 前区間にない列車がない場合"
                    );
                    for (i, between_train) in between_trains.iter().enumerate() {
                        let before_index = before_orders
                            .order
                            .iter()
                            .position(|t| t == between_train)
                            .unwrap();
                        if before_index < before_segment_order {
                            // 前区間で先行列車より前にあった場合
                            continue;
                        }

                        return Ok(Some(next_before_preceding_train_index + 1 + i));
                    }
                    Ok(Some(next_before_following_train_index))
                }
                // 次区間で先行列車・続行列車が存在しない
                (None, None) => {
                    println!(
                        "先行列車・続行列車が前区間に存在する, 次区間で先行列車・続行列車が存在しない"
                    );
                    Ok(None)
                }
                // 次区間で先行列車が存在せず、続行列車が存在する
                (None, Some(next_before_following_train_index)) => {
                    // 次区間における続行列車の1本前の列車
                    let before_train = if next_before_following_train_index == 0 {
                        None
                    } else {
                        next_orders.order.get(next_before_following_train_index - 1)
                    };

                    if let Some(before_train) = before_train {
                        let is_exist_before_train = before_orders.order.contains(before_train);
                        if is_exist_before_train {
                            println!(
                                "先行列車・続行列車が前区間に存在する, 次区間で先行列車が存在せず、続行列車が存在する, 続行列車の1本前が前区間に存在する"
                            );
                            // 続行列車の1本前が前区間に存在する
                            Ok(Some(next_before_following_train_index))
                        } else {
                            println!(
                                "先行列車・続行列車が前区間に存在する, 次区間で先行列車が存在せず、続行列車が存在する, 続行列車の1本前が前区間に存在しない"
                            );
                            // 続行列車の1本前が前区間に存在しない
                            Ok(None)
                        }
                    } else {
                        println!(
                            "先行列車・続行列車が前区間に存在する, 次区間で先行列車が存在せず、続行列車が存在する, 続行列車の1本前が存在しない"
                        );
                        // 続行列車の1本前が存在しない
                        Ok(Some(next_before_following_train_index))
                    }
                }
                // 次区間で先行列車が存在し、続行列車が存在しない
                (Some(next_before_preceding_train_index), None) => {
                    // 次区間における先行列車の1本後の列車
                    let after_train = next_orders.order.get(next_before_preceding_train_index + 1);

                    if let Some(after_train) = after_train {
                        let is_exist_after_train = before_orders.order.contains(after_train);
                        if is_exist_after_train {
                            // 先行列車の1本後が前区間に存在する
                            println!(
                                "先行列車・続行列車が前区間に存在する, 次区間で先行列車が存在し、続行列車が存在しない, 先行列車の1本後が前区間に存在する"
                            );
                            Ok(Some(next_before_preceding_train_index + 1))
                        } else {
                            println!(
                                "先行列車・続行列車が前区間に存在する, 次区間で先行列車が存在し、続行列車が存在しない, 先行列車の1本後が前区間に存在しない"
                            );
                            // 先行列車の1本後が前区間に存在しない
                            Ok(None)
                        }
                    } else {
                        println!(
                            "先行列車・続行列車が前区間に存在する, 次区間で先行列車が存在し、続行列車が存在しない, 先行列車の1本後が前区間に存在しない"
                        );
                        // 先行列車の1本後が存在しない
                        Ok(Some(next_before_preceding_train_index + 1))
                    }
                }
            }
        }
    }
}

fn get_train_segment(
    root: &DiagramRoot,
    train: TrainId,
) -> Result<Vec<(usize, LineSegmentId, bool)>, ModelError> {
    let train = root.trains.get(&train).ok_or(ModelError::ObjectNotFound)?;
    let mut result: Vec<(usize, LineSegmentId, bool)> = vec![];

    for segment in train.template_segments.iter().enumerate() {
        let segments = root
            .template_trains
            .get(&segment.1.template_train_id)
            .ok_or(ModelError::ObjectNotFound)?
            .get_filtered_segment(segment.1.start_station_id, segment.1.end_station_id)?;
        for ls in &segments.1 {
            result.push((segment.0, ls.0.segment_id, ls.0.is_reversed));
        }
    }

    Ok(result)
}

/// 列車の発車順序を頭から求め、列車順序を挿入する関数
pub fn insert_train_order(
    root: &mut DiagramRoot,
    // 対象時刻表ID
    timetable_id: TimetableId,
    // 対象列車ID
    target_train_id: TrainId,
) -> Result<(), ModelError> {
    // 元の列車
    let original_target_train = root
        .trains
        .get(&target_train_id)
        .ok_or(ModelError::ObjectNotFound)?
        .clone();
    let segments: Vec<_> = get_train_segment(root, target_train_id)?;
    if segments.is_empty() {
        return Err(ModelError::Empty);
    }
    root.trains
        .get_mut(&target_train_id)
        .ok_or(ModelError::ObjectNotFound)?
        .template_segments = vec![];

    let mut nodes = crate::make_node_diff::make_node(root, timetable_id)?;
    let mut node_array = crate::sort_diff::sort_node(&nodes)?;
    let mut times = crate::ripple_diff::ripple_node_diff(&nodes, &node_array);

    // 前区間における順序
    let mut order: usize;
    let mut departure_time = original_target_train.start_departure_time;

    // 初回
    {
        let segment = segments.first().unwrap();
        order =
            get_starting_order_index(root, &nodes, &times, segment.1, segment.2, departure_time)?;
        println!("1. 初回");
        println!("順序: {:?}", order);

        let template_segment = original_target_train.template_segments.first().unwrap();

        let line_segment = root
            .segments
            .get(&segment.1)
            .ok_or(ModelError::ObjectNotFound)?;
        let (start_station, end_station) = if segment.2 {
            (line_segment.end_station, line_segment.start_station)
        } else {
            (line_segment.start_station, line_segment.end_station)
        };

        let timetable = root
            .timetables
            .get_mut(&timetable_id)
            .ok_or(ModelError::ObjectNotFound)?;
        let orders = timetable
            .segment_train_orders
            .get_mut(&line_segment.id)
            .ok_or(ModelError::ObjectNotFound)?;
        let orders = if segment.2 {
            &mut orders.1
        } else {
            &mut orders.0
        };
        orders.order.insert(order, target_train_id);

        root.trains
            .get_mut(&target_train_id)
            .ok_or(ModelError::ObjectNotFound)?
            .template_segments
            .push(TemplateSegment {
                template_train_id: template_segment.template_train_id,
                start_station_id: start_station,
                end_station_id: end_station,
            });
    }

    let mut before_segment_id = segments.first().unwrap().1;
    let mut before_segment_reversed = segments.first().unwrap().2;

    for (template_idx, segment, is_reversed) in &segments[1..] {
        let current_order = get_next_order_index(
            root,
            timetable_id,
            before_segment_id,
            before_segment_reversed,
            *segment,
            *is_reversed,
            order,
        )?;

        // 自区間の順序を同定する
        if let Some(current_order) = current_order {
            println!("1. 前区間より順序が同定された");
            order = current_order;
            println!("順序: {:?}", order);
        } else {
            println!("1. 前区間より順序が同定できなかった");
            nodes = crate::make_node_diff::make_node(root, timetable_id)?;
            node_array = crate::sort_diff::sort_node(&nodes)?;
            times = crate::ripple_diff::ripple_node_diff(&nodes, &node_array);

            let before_arrival_station = if before_segment_reversed {
                root.get_segment(before_segment_id)
                    .ok_or(ModelError::ObjectNotFound)?
                    .start_station
            } else {
                root.get_segment(before_segment_id)
                    .ok_or(ModelError::ObjectNotFound)?
                    .end_station
            };
            // 前区間の到着時刻
            let before_arrival_time = times[nodes
                .nodes
                .iter()
                .find(|node| {
                    node.node_type == NodeType::Arrival
                        && node.station_id == before_arrival_station
                        && node.train_id == target_train_id
                })
                .ok_or(ModelError::ObjectNotFound)?
                .node_id
                .0];
            departure_time = before_arrival_time + Time::new(0, 0, 30);

            order = get_starting_order_index(
                root,
                &nodes,
                &times,
                *segment,
                *is_reversed,
                departure_time,
            )?;
            println!("順序: {:?}", order);
        }

        let timetable = root
            .timetables
            .get_mut(&timetable_id)
            .ok_or(ModelError::ObjectNotFound)?;
        let orders = timetable
            .segment_train_orders
            .get_mut(segment)
            .ok_or(ModelError::ObjectNotFound)?;
        let orders = if *is_reversed {
            &mut orders.1
        } else {
            &mut orders.0
        };
        orders.order.insert(order, target_train_id);

        // 自区間の情報を列車に挿入
        let train = root
            .trains
            .get_mut(&target_train_id)
            .ok_or(ModelError::ObjectNotFound)?;
        if *template_idx != train.template_segments.len() - 1 {
            // 新規TemplateSegmentを挿入
            let line_segment = root
                .segments
                .get(segment)
                .ok_or(ModelError::ObjectNotFound)?;

            let (start_station, end_station) = if *is_reversed {
                (line_segment.end_station, line_segment.start_station)
            } else {
                (line_segment.start_station, line_segment.end_station)
            };

            train.template_segments.push(TemplateSegment {
                template_train_id: original_target_train
                    .template_segments
                    .get(*template_idx)
                    .ok_or(ModelError::ObjectNotFound)?
                    .template_train_id,
                start_station_id: start_station,
                end_station_id: end_station,
            });
        } else {
            // 既存TemplateSegmentに挿入
            let before_template_segment = train
                .template_segments
                .last_mut()
                .ok_or(ModelError::Error)?;
            let end_station_id = if *is_reversed {
                root.segments
                    .get(segment)
                    .ok_or(ModelError::ObjectNotFound)?
                    .start_station
            } else {
                root.segments
                    .get(segment)
                    .ok_or(ModelError::ObjectNotFound)?
                    .end_station
            };
            before_template_segment.end_station_id = end_station_id;
        }

        before_segment_id = *segment;
        before_segment_reversed = *is_reversed;
    }

    Ok(())
}

#[test]
fn test() {
    use weaverail_model::model::train::Train;
    use weaverail_model::test_data::diagram_root::get_test_data_shortly;

    let mut test_data = get_test_data_shortly();
    let timetable_id = test_data
        .root
        .timetables
        .values()
        .find(|_| true)
        .unwrap()
        .id;

    let mut add_train = Train::new(TrainId(test_data.root.id_issuer.next()), timetable_id);
    add_train.template_segments = vec![TemplateSegment {
        template_train_id: test_data
            .root
            .find_template_train_by_name("神姫線下り-普通")
            .unwrap()
            .id,
        start_station_id: test_data.root.find_station_by_name("湊川").unwrap().id,
        end_station_id: test_data.root.find_station_by_name("姫路").unwrap().id,
    }];
    add_train.start_departure_time = Time::new(11, 0, 0);
    test_data
        .root
        .trains
        .insert(add_train.id, add_train.clone());

    let _ = insert_train_order(&mut test_data.root, timetable_id, add_train.id);

    let nodes = crate::make_node_diff::make_node(&test_data.root, timetable_id).unwrap();
    let node_array = crate::sort_diff::sort_node(&nodes).unwrap();
    let times: Vec<Time> = crate::ripple_diff::ripple_node_diff(&nodes, &node_array);
    let result = crate::time_result_diff::get_time_result_diff(
        &test_data.root,
        timetable_id,
        &nodes,
        &times,
    );

    println!("result: {:?}", result);
}
