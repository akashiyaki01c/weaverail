//! # sort
//!
//! Kahn (1962) のトポロジカルソートを使用し、有向グラフを一次元に整列するモジュール
//!
//! 閉路を検出した場合panicを起こす

use std::collections::VecDeque;

use weaverail_model::{
    error::ModelError,
    result_weft::{NodeId, WeftNode},
};

/// ノードをトポロジカル順に整列する。
/// 計算量は O(|V|+|E|)
/// アルゴリズムは Kahn (1962) のトポロジカルソートを使用
/// 閉路を検出した場合は [`ModelError::DiagramGraphClosedPath`] を返す。
pub fn sort_node<'a>(nodes: &'a Vec<&'a WeftNode>) -> Result<Vec<&'a WeftNode>, ModelError> {
    let count = nodes.len();
    // 入次数
    let mut num_input: Vec<usize> = vec![0; count];

    for node in nodes {
        for edge in &node.edges {
            num_input[edge.0.0] += 1;
        }
    }

    let mut que = VecDeque::new();
    for (i, v) in num_input.iter().enumerate().take(count) {
        if *v == 0 {
            que.push_back(i);
        }
    }

    let mut answer = Vec::with_capacity(count);
    while !que.is_empty() {
        let node = {
            let node_index = que.front().ok_or(ModelError::DiagramGraphError)?;
            let node_id = NodeId::new(*node_index);
            nodes[node_id.0]
        };
        que.pop_front();
        answer.push(node);
        for edge in &node.edges {
            num_input[edge.0.0] -= 1;
            if num_input[edge.0.0] == 0 {
                que.push_back(edge.0.0);
            }
        }
    }

    if answer.len() != count {
        Err(ModelError::DiagramGraphClosedPath)
    } else {
        Ok(answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smallvec::smallvec;
    use weaverail_model::{model::Time, result_weft::NodeId};

    fn node(id: usize, edges: &[(usize, Time)]) -> WeftNode {
        WeftNode {
            node_id: NodeId::new(id),
            edges: edges
                .iter()
                .map(|(target, time)| (NodeId::new(*target), *time))
                .collect(),
            ..Default::default()
        }
    }

    #[test]
    fn sort_node_handles_empty_and_linear_graphs() {
        let empty: Vec<&WeftNode> = vec![];
        assert_eq!(sort_node(&empty).unwrap(), Vec::<&WeftNode>::new());

        let first = node(0, &[(1, Time::new(0, 0, 1))]);
        let second = node(1, &[]);
        let nodes = vec![&first, &second];
        let sorted = sort_node(&nodes).unwrap();
        assert_eq!(
            sorted
                .iter()
                .map(|value| value.node_id.0)
                .collect::<Vec<_>>(),
            vec![0, 1]
        );
    }

    #[test]
    fn sort_node_reports_cycles() {
        let first = node(0, &[(1, Time::default())]);
        let second = node(1, &[(0, Time::default())]);
        let nodes = vec![&first, &second];
        assert_eq!(sort_node(&nodes), Err(ModelError::DiagramGraphClosedPath));
    }
}
