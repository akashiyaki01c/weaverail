//! `ripple` 列車時刻ノードの実際の時刻を計算するモジュール
//!
//! 有向非巡回グラフ上の最長経路問題としてグラフを解くアルゴリズムを採用している

use weaverail_model::model::time::Time;

use crate::WeftNode;

/// トポロジカル順に並んだノードから各ノードの最早時刻を計算する。
///
/// 各エッジの重みを制約時間として、始点からの最長経路を求める。
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

#[cfg(test)]
mod tests {
    use super::*;
    use weaverail_model::result_weft::NodeId;

    #[test]
    fn ripple_time_uses_longest_path() {
        let first = WeftNode {
            node_id: NodeId::new(0),
            edges: smallvec::smallvec![
                (NodeId::new(1), Time::new(0, 0, 5)),
                (NodeId::new(2), Time::new(0, 0, 2)),
            ],
            ..Default::default()
        };
        let second = WeftNode {
            node_id: NodeId::new(1),
            edges: smallvec::smallvec![(NodeId::new(2), Time::new(0, 0, 4))],
            ..Default::default()
        };
        let third = WeftNode {
            node_id: NodeId::new(2),
            ..Default::default()
        };
        let nodes = vec![&first, &second, &third];
        assert_eq!(
            ripple_time(&nodes),
            vec![
                Time::default(),
                Time::new_from_total_second(5),
                Time::new_from_total_second(9)
            ]
        );
    }
}
