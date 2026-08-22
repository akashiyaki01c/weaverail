//! `ripple` 列車時刻ノードの実際の時刻を計算するモジュール
//!
//! 有向非巡回グラフ上の最長経路問題としてグラフを解くアルゴリズムを採用している

use weaverail_model::{model::time::Time, result_weft::WeftTempObj};

/// 最適化済みノードグラフから各ノードの最早時刻を計算する。
///
/// `sorted_node_index` は `tmp_obj.nodes` の有効な添字列である必要がある。
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

#[cfg(test)]
mod tests {
    use super::*;
    use weaverail_model::result_weft::{NodeId, WeftNode};

    #[test]
    fn ripple_node_diff_uses_longest_path() {
        let graph = WeftTempObj {
            nodes: vec![
                WeftNode {
                    node_id: NodeId::new(0),
                    edges: smallvec::smallvec![
                        (NodeId::new(1), Time::new(0, 0, 5)),
                        (NodeId::new(2), Time::new(0, 0, 2)),
                    ],
                    ..Default::default()
                },
                WeftNode {
                    node_id: NodeId::new(1),
                    edges: smallvec::smallvec![(NodeId::new(2), Time::new(0, 0, 4))],
                    ..Default::default()
                },
                WeftNode {
                    node_id: NodeId::new(2),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        assert_eq!(
            ripple_node_diff(&graph, &[0, 1, 2]),
            vec![
                Time::default(),
                Time::new_from_total_second(5),
                Time::new_from_total_second(9)
            ]
        );
    }
}
