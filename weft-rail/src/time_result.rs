use weaverail_model::model::{DiagramRoot, time::Time, timetable::TimetableId};

use crate::{NodeType, ResultWeftTime, ResultWeftTrain, StopType, WeftNode};

pub(crate) fn get_time_result(
    diagram_root: &DiagramRoot,
    timetable_id: TimetableId,
    nodes: Vec<&WeftNode>,
    times: &Vec<Time>,
) -> Vec<ResultWeftTrain> {
    let timetable = diagram_root.timetables.get(&timetable_id).unwrap();
    let mut result = Vec::new();

    for train in timetable.trains.values() {
        let mut result_train = ResultWeftTrain {
            train_id: train.id,
            times: vec![],
        };
        for station_id in diagram_root.get_stations(train) {
            let arrival_segment_node_id = nodes.iter().find(|node| {
                node.train_id == train.id
                    && node.station_id == station_id
                    && node.node_type == NodeType::Arrival
            });
            let departure_segment_node_id = nodes.iter().find(|node| {
                node.train_id == train.id
                    && node.station_id == station_id
                    && node.node_type == NodeType::Departure
            });
            let arrival_segment_time = if let Some(id) = arrival_segment_node_id {
                Some(times[id.node_id.0])
            } else {
                None
            };
            let arrival_segment_id = if let Some(id) = arrival_segment_node_id {
                Some(id.segment_id)
            } else {
                None
            };
            let departure_segment_time = if let Some(id) = departure_segment_node_id {
                Some(times[id.node_id.0])
            } else {
                None
            };
            let departure_segment_id = if let Some(id) = departure_segment_node_id {
                Some(id.segment_id)
            } else {
                None
            };
            let stop_type: StopType = {
                if let Some(id) = arrival_segment_node_id {
                    id.stop_type
                } else if let Some(id) = departure_segment_node_id {
                    id.stop_type
                } else {
                    panic!()
                }
            };

            let time = ResultWeftTime {
                before_segment_id: arrival_segment_id,
                next_segment_id: departure_segment_id,
                arrival_time: arrival_segment_time,
                departure_time: departure_segment_time,
                stop_type: stop_type,
                station_id,
                train_id: train.id,
            };
            result_train.times.push(time);
        }
        result.push(result_train);
    }

    result
}
