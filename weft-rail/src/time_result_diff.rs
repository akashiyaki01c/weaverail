use rustc_hash::{FxBuildHasher, FxHashMap};
use weaverail_model::{
    model::{
        DiagramRoot,
        station::StationId,
        time::Time,
        timetable::TimetableId,
        train::{Train, TrainId},
    }, result_weft::{NodeType, ResultWeftTime, ResultWeftTrain, StopType, WeftTempObj},
};

#[derive(Clone, PartialEq, Default, Eq, Hash, Copy)]
pub struct LookupNodeKey(u64);
impl LookupNodeKey {
    pub fn new(train_id: TrainId, station_id: StationId, node_type: NodeType) -> Self {
        let raw_train_id = train_id.0.0;
        let raw_station_id = station_id.0.0;
        let raw_node_type = match node_type {
            NodeType::Arrival => 1,
            NodeType::Departure => 2,
            NodeType::Root => 3,
        };
        Self((raw_node_type as u64) << 62 | (raw_station_id as u64) << 31 | raw_train_id as u64)
    }
}

pub fn get_time_result_diff(
    diagram_root: &DiagramRoot,
    timetable_id: TimetableId,
    obj: &WeftTempObj,
    times: &[Time],
) -> Vec<ResultWeftTrain> {
    let _timetable = diagram_root.timetables.get(&timetable_id).unwrap();
    let trains: Vec<&Train> = diagram_root
        .trains
        .values()
        .filter(|train| train.timetable_id == timetable_id)
        .collect();
    let mut result = Vec::with_capacity(trains.len());

    let mut node_map =
        FxHashMap::with_capacity_and_hasher(obj.nodes.len(), FxBuildHasher);
    for node in &obj.nodes {
        node_map.insert(
            LookupNodeKey::new(node.train_id, node.station_id, node.node_type),
            node.clone(),
        );
    }

    for train in trains {
        let mut result_train = ResultWeftTrain {
            train_id: train.id,
            times: vec![],
        };
        for station_id in diagram_root.get_stations(train) {
            let arrival_node =
                node_map.get(&LookupNodeKey::new(train.id, station_id, NodeType::Arrival));
            let departure_node = node_map.get(&LookupNodeKey::new(
                train.id,
                station_id,
                NodeType::Departure,
            ));

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

    result
}
