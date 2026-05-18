use weaverail_model::model::time::Time;

use crate::make_node_diff::WeftTempObj;

pub fn ripple_node_diff(tmp_obj: &WeftTempObj, sorted_node_index: &[usize]) -> Vec<Time> {
    let mut times = vec![Time::new(0, 0, 0); tmp_obj.nodes.len()];

    for node_index in 0..sorted_node_index.len() {
		let node = &tmp_obj.nodes[sorted_node_index[node_index]];
        let current_time = times[sorted_node_index[node_index]];
        for (target_id, weight) in &node.edges {
            let next_node_index = target_id.0;
            let arrival_time = current_time + *weight;
            if arrival_time > times[next_node_index] {
                times[next_node_index] = arrival_time;
            }
        }
    }

    times
}
