//! 列車順序の同定アルゴリズムを実装するモジュール

use weaverail_model::{
    error::ModelError,
    model::{DiagramRoot, line_segment::LineSegmentId, time::Time, timetable::TimetableId},
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
            .end_station(&root)?
            .id
    } else {
        root.segments
            .get(&segment_id)
            .ok_or(ModelError::ObjectNotFound)?
            .start_station(&root)?
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
        (None, None) => return Ok(None),
        // 先行列車が前区間に存在せず、続行列車が前区間に存在する
        (None, Some(_before_following_train)) => {
            match next_before_following_train_index {
                Some(next_before_following_train_index) => {
                    // 続行列車が次区間にも存在する
                    if next_before_following_train_index == 0 {
                        // 次区間において、続行列車の前に列車が存在しない場合
                        return Ok(Some(next_before_following_train_index));
                    } else {
                        // 次区間において、続行列車の前に列車が存在する場合
                        return Ok(None);
                    }
                },
                None => {
                    // 続行列車が次区間に存在しない
                    return Ok(None);
                },
            }
        },
        // 先行列車が前区間に存在し、続行列車が前区間に存在しない
        (Some(_before_preceding_train), None) => {
            match next_before_preceding_train_index {
                Some(next_before_preceding_train_index) => {
                    // 先行列車が次区間にも存在する
                    if next_before_preceding_train_index == next_orders.order.len()-1 {
                        // 次区間において、先行列車の後に列車が存在しない場合
                        return Ok(Some(next_before_preceding_train_index+1));
                    } else {
                        // 次区間において、先行列車の後に列車が存在する場合
                        return Ok(None);
                    }
                },
                None => {
                    // 先行列車が次区間に存在しない
                    return Ok(None)
                },
            }
        },
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
                        // 次区間で先行列車→続行列車の順に連続している
                        return Ok(Some(next_before_following_train_index));
                    } else if next_before_following_train_index < next_before_preceding_train_index
                    {
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
                        // 前区間にない列車がある場合
                        return Ok(None);
                    }

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
                    return Ok(Some(next_before_following_train_index));
                }
                // 次区間で先行列車・続行列車が存在しない
                (None, None) => return Ok(None),
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
                            // 続行列車の1本前が前区間に存在する
                            return Ok(Some(next_before_following_train_index));
                        } else {
                            // 続行列車の1本前が前区間に存在しない
                            return Ok(None);
                        }
                    } else {
                        // 続行列車の1本前が存在しない
                        return Ok(Some(next_before_following_train_index));
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
                            return Ok(Some(next_before_preceding_train_index + 1));
                        } else {
                            // 先行列車の1本後が前区間に存在しない
                            return Ok(None);
                        }
                    } else {
                        // 先行列車の1本後が存在しない
                        return Ok(Some(next_before_preceding_train_index + 1));
                    }
                }
            }
        }
    }
}

#[test]
fn test() {
    use weaverail_model::test_data::diagram_root::get_test_data_shortly;

    let test_data = get_test_data_shortly();
    let timetable_id = test_data
        .root
        .timetables
        .values()
        .find(|_| true)
        .unwrap()
        .id;

    let nodes = crate::make_node_diff::make_node(&test_data.root, timetable_id).unwrap();
    let node_array = crate::sort_diff::sort_node(&nodes).unwrap();
    let times: Vec<Time> = crate::ripple_diff::ripple_node_diff(&nodes, &node_array);

    let result = get_starting_order_index(
        &test_data.root,
        &nodes,
        &times,
        test_data
            .root
            .find_segment_by_name("湊川", "会下山")
            .unwrap()
            .segment_id,
        false,
        Time::new(12, 5, 0),
    );

    println!("result: {:?}", result);
}
