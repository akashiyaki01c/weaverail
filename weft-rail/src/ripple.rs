use weaverail_model::model::time::Time;

use crate::{NodeType, WeftNode};

/// ノードを実際に計算する関数
pub(crate) fn ripple_time(sorted_nodes: &Vec<&WeftNode>) -> Vec<Time> {
    let mut times = vec![Time::new(0, 0, 0); sorted_nodes.len()];

    for node in sorted_nodes {
        let current_time = times[node.node_id.0];
        for (target_id, weight) in &node.edges {
            let next_node_index = target_id.0;
            let arrival_time = current_time + *weight;
            if arrival_time > times[next_node_index] {
                times[next_node_index] = arrival_time;
            }
        }
    }

    for node in sorted_nodes {
        if node.node_type == NodeType::Root {
            println!("root edges: {:?}", node.edges); // ← 追加
        }
        // ...
    }

    times
}
