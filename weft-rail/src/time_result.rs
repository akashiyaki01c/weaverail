use std::collections::HashMap;

use weaverail_model::{
    error::ModelError,
    model::{DiagramRoot, time::Time, timetable::TimetableId, train::Train},
    result_weft::{NodeType, ResultWeftTime, StopType},
};

use crate::{ResultWeftTrain, WeftNode};

pub fn get_time_result(
    diagram_root: &DiagramRoot,
    timetable_id: TimetableId,
    nodes: Vec<&WeftNode>,
    times: &[Time],
) -> Result<Vec<ResultWeftTrain>, ModelError> {
    let _timetable = diagram_root
        .timetables
        .get(&timetable_id)
        .ok_or(ModelError::ObjectNotFound)?;
    let trains: Vec<&Train> = diagram_root
        .trains
        .values()
        .filter(|train| train.timetable_id == timetable_id)
        .collect();
    let mut result = Vec::with_capacity(trains.len());

    let mut node_map = HashMap::with_capacity(nodes.len());
    for node in &nodes {
        node_map.insert((node.train_id, node.station_id, node.node_type), *node);
    }

    for train in trains {
        let mut result_train = ResultWeftTrain {
            train_id: train.id,
            times: vec![],
        };
        for station_id in diagram_root.get_train_stations(train)? {
            let arrival_node = node_map.get(&(train.id, station_id, NodeType::Arrival));
            let departure_node = node_map.get(&(train.id, station_id, NodeType::Departure));

            let arrival_segment_time = arrival_node.map(|id| times[id.node_id.0]);
            let arrival_segment_id = arrival_node.map(|id| id.segment_id);
            let departure_segment_time = departure_node.map(|id| times[id.node_id.0]);
            let departure_segment_id = departure_node.map(|id| id.segment_id);
            let stop_type: StopType = {
                if let Some(id) = arrival_node {
                    id.stop_type.clone()
                } else if let Some(id) = departure_node {
                    id.stop_type.clone()
                } else {
                    panic!()
                }
            };
            let arrival_segment_time = if let (Some(arrival), Some(departure)) =
                (arrival_segment_time, departure_segment_time)
            {
                if stop_type == StopType::Pass && departure != arrival {
                    Some(departure)
                } else {
                    Some(arrival)
                }
            } else {
                arrival_segment_time
            };

            let time = ResultWeftTime {
                before_segment_id: arrival_segment_id,
                next_segment_id: departure_segment_id,
                arrival_time: arrival_segment_time,
                departure_time: departure_segment_time,
                stop_type,
                station_id,
                train_id: train.id,
            };
            result_train.times.push(time);
        }
        result.push(result_train);
    }

    Ok(result)
}
