use std::collections::HashMap;
use std::fmt::Write;

use weaverail_model::{
    diagram_logical_coord::{DiagramLogicalConvert, DiagramLogicalCoord},
    model::{
        DiagramRoot,
        diagram_view_settings::{DiagramViewSegment, DiagramViewSettingsId},
        line_segment::LineSegmentId,
        station::StationId,
        time::Time,
        timetable::TimetableId,
        train::TrainId,
    },
    result_svg::{ResultSvg, ResultSvgTrain},
    result_warp::{ResultWarpCoords, ResultWarpStations},
    result_weft::{NodeType, WeftTempObj},
};

const DEFAULT_BLANK_TIME: Time = Time::new(0, 2, 0);

/// ダイヤグラム上のY座標を求める
pub fn warp_coords(
    root: &DiagramRoot,
    settings_id: DiagramViewSettingsId,
) -> HashMap<LineSegmentId, ResultWarpCoords> {
    let settings = root.diagram_view_settings.get(&settings_id).expect("");
    let mut result = HashMap::new();
    let mut current_y: f64 = 0.0;
    for segment in &settings.segments {
        match segment {
            DiagramViewSegment::Black { scale } => {
                current_y += DEFAULT_BLANK_TIME.total_second() as f64 * *scale as f64;
            }
            DiagramViewSegment::StationBetween { segment } => {
                let upper_y = current_y;
                let segments = root.get_template_segments(segment.segment_id);
                let time = segments
                    .iter()
                    .max_by(|x, y| x.running_time.cmp(&y.running_time));
                let time = if time.is_none() {
                    DEFAULT_BLANK_TIME
                } else {
                    time.unwrap().running_time
                };
                current_y += time.total_second() as f64;
                let lower_y = current_y;

                result.insert(
                    segment.segment_id,
                    ResultWarpCoords {
                        upper_y,
                        lower_y,
                        segment_id: segment.segment_id,
                        is_reversed: segment.is_reversed,
                    },
                );
            }
        }
    }
    result
}

/// 駅座標情報を求める関数
pub fn warp_stations(
    root: &DiagramRoot,
    settings_id: DiagramViewSettingsId,
) -> Vec<ResultWarpStations> {
    let warp = warp_coords(root, settings_id);
    let mut result: Vec<ResultWarpStations> = vec![];

    for coord in warp.values() {
        let segment = root.segments.get(&coord.segment_id).expect("");

        let start_sta = if coord.is_reversed {
            segment.end_station(root).unwrap()
        } else {
            segment.start_station(root).unwrap()
        };
        if !result.iter().any(|v| v.y_coord == coord.upper_y) {
            result.push(ResultWarpStations {
                y_coord: coord.upper_y,
                station_id: start_sta.id,
                name: start_sta.name.to_string(),
            });
        }

        let end_sta = if coord.is_reversed {
            segment.start_station(root).unwrap()
        } else {
            segment.end_station(root).unwrap()
        };
        if !result.iter().any(|v| v.y_coord == coord.lower_y) {
            result.push(ResultWarpStations {
                y_coord: coord.lower_y,
                station_id: end_sta.id,
                name: end_sta.name.to_string(),
            });
        }
    }

    result
}

fn get_coord(
    root: &DiagramRoot,
    coords: &HashMap<LineSegmentId, ResultWarpCoords>,
    segment_id: LineSegmentId,
    station_id: StationId,
) -> f64 {
    let segment = root.segments.get(&segment_id).unwrap();
    let coord = coords.get(&segment_id).unwrap();

    let mut result = false;
    if segment.start_station(root).unwrap().id == station_id {
        result = !result;
    }
    if coord.is_reversed {
        result = !result;
    }

    if result { coord.upper_y } else { coord.lower_y }
}

pub fn get_svg(
    root: &DiagramRoot,
    timetable_id: TimetableId,
    obj: &WeftTempObj,
    node_array: &[usize],
    times: &[Time],
    coords: &HashMap<LineSegmentId, ResultWarpCoords>,
    settings: DiagramLogicalConvert,
    start_time: Time,
    end_time: Time,
) -> ResultSvg {
    let mut nodes_by_train: HashMap<TrainId, Vec<usize>> = HashMap::new();
    for &actual_index in node_array {
        let train_id = obj.nodes[actual_index].train_id;
        nodes_by_train
            .entry(train_id)
            .or_default()
            .push(actual_index);
    }

    let result = root
        .trains
        .values()
        .filter(|train| train.timetable_id == timetable_id)
        .map(|train| {
            let node_indexes = nodes_by_train
                .get(&train.id)
                .map(|v| v.as_slice())
                .unwrap_or(&[]);

            let mut path = String::with_capacity(node_indexes.len() * 32);

            // 前駅発車時刻〜次駅到着時刻、現在駅到着時刻〜現在駅発車時刻の対
            for window in node_indexes.windows(2) {
                let (bi, ci) = (window[0], window[1]);
                let (before_node, before_time): (&_, _) = (&obj.nodes[bi], times[bi]);
                let (current_node, current_time): (&_, _) = (&obj.nodes[ci], times[ci]);

                if before_time < start_time && current_time < start_time {
                    continue;
                }
                if before_time > end_time && current_time > end_time {
                    break;
                }

                let before_y =
                    get_coord(root, coords, before_node.segment_id, before_node.station_id);
                let current_y = get_coord(
                    root,
                    coords,
                    current_node.segment_id,
                    current_node.station_id,
                );

                let before_coord = settings.convert(DiagramLogicalCoord::new(
                    before_time.total_second() as f64,
                    before_y,
                ));
                let current_coord = settings.convert(DiagramLogicalCoord::new(
                    current_time.total_second() as f64,
                    current_y,
                ));

                if before_coord == current_coord {
                    continue;
                }

                if path.is_empty() {
                    let _ = write!(
                        path,
                        "M {},{} L {},{}",
                        before_coord.x, before_coord.y, current_coord.x, current_coord.y
                    );
                } else if before_node.node_type == NodeType::Arrival
                    && current_node.node_type == NodeType::Departure
                    && before_y != current_y
                {
                    let _ = write!(path, " M {},{}", current_coord.x, current_coord.y);
                } else {
                    let _ = write!(path, " L {},{}", current_coord.x, current_coord.y);
                }
            }

            ResultSvgTrain {
                train_id: train.id,
                path_string: path,
            }
        })
        .filter(|v| !v.path_string.is_empty())
        .collect();

    ResultSvg { trains: result }
}
