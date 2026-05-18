use std::collections::HashMap;

use weaverail_model::{
    diagram_logical_coord::{DiagramLogicalConvert, DiagramLogicalCoord},
    model::{
        DiagramRoot,
        diagram_view_settings::{DiagramViewSegment, DiagramViewSettings},
        line_segment::LineSegmentId,
        station::StationId,
        time::Time,
        timetable::TimetableId,
        train::Train,
    },
    result_svg::{ResultSvg, ResultSvgTrain},
};
use weft_rail::{WeftNode, make_node_diff::WeftTempObj};

const DEFAULT_BLANK_TIME: Time = Time::new(0, 2, 0);

pub struct ResultWarpCoords {
    pub upper_y: f64,
    pub lower_y: f64,
    pub segment_id: LineSegmentId,
    pub is_reversed: bool,
}

/// ダイヤグラム上のY座標を求める
pub fn warp_coords(
    root: &DiagramRoot,
    settings: &DiagramViewSettings,
) -> HashMap<LineSegmentId, ResultWarpCoords> {
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

fn get_coord(
    root: &DiagramRoot,
    coords: &HashMap<LineSegmentId, ResultWarpCoords>,
    segment_id: LineSegmentId,
    station_id: StationId,
) -> f64 {
    let segment = root.segments.get(&segment_id).unwrap();
    let coord = coords.get(&segment_id).unwrap();

    if segment.start_station(&root).unwrap().id == station_id {
        if coord.is_reversed {
            coord.lower_y
        } else {
            coord.upper_y
        }
    } else {
        if coord.is_reversed {
            coord.upper_y
        } else {
            coord.lower_y
        }
    }
}

pub fn get_svg(
    root: &DiagramRoot,
    timetable_id: TimetableId,
    obj: &WeftTempObj,
    node_array: &Vec<usize>,
    times: &Vec<Time>,
    coords: &HashMap<LineSegmentId, ResultWarpCoords>,
    settings: DiagramLogicalConvert,
    start_time: Time,
    end_time: Time,
) -> ResultSvg {
    let trains: Vec<&Train> = root
        .trains
        .values()
        .filter(|train| train.timetable_id == timetable_id)
        .collect();
    let mut result = Vec::new();

    for train in trains {
        let mut node_indexes = vec![];
        for i in 0..node_array.len() {
            let actual_index = node_array[i];
            let node = &obj.nodes[actual_index];
            if node.train_id == train.id {
                node_indexes.push(actual_index);
            }
        }

        let mut values = vec![];
        for &index in &node_indexes {
            values.push((&obj.nodes[index], times[index]));
        }

        let mut strs = vec![];
        for v in values.windows(2) {
            let before = v[0];
            let current = v[1];

            if before.1 < start_time && current.1 < start_time {
                continue;
            }
            if before.1 > end_time && current.1 > end_time {
                break;
            }

            let before_y = get_coord(root, coords, before.0.segment_id, before.0.station_id);
            let current_y = get_coord(root, coords, current.0.segment_id, current.0.station_id);

            let before_coord = DiagramLogicalCoord::new(before.1.total_second() as f64, before_y);
            let current_coord =
                DiagramLogicalCoord::new(current.1.total_second() as f64, current_y);

            let before_coord = settings.convert(before_coord);
            let current_coord = settings.convert(current_coord);

            if strs.is_empty() {
                strs.push(format!(
                    "M {},{} L {},{}",
                    before_coord.x, before_coord.y, current_coord.x, current_coord.y
                ));
            } else {
                strs.push(format!(
                    "L {},{} L {},{}",
                    before_coord.x, before_coord.y, current_coord.x, current_coord.y
                ));
            }
        }

        result.push(ResultSvgTrain {
            train_id: train.id,
            path_string: strs.join(" "),
        });
    }

    ResultSvg { trains: result }
}
