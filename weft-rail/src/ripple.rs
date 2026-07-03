//! `ripple` 列車時刻ノードの実際の時刻を計算するモジュール
//! 
//! 有向非巡回グラフ上の最長経路問題としてグラフを解くアルゴリズムを採用している

use weaverail_model::model::time::Time;

use crate::WeftNode;

/// 列車時刻ノードの実際の時刻を関数
pub fn ripple_time(sorted_nodes: &Vec<&WeftNode>) -> Vec<Time> {
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

    times
}
