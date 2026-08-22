//! # sort_diff
//!
//! Kahn (1962) のトポロジカルソートを使用し、有向グラフを一次元に整列するモジュール (最適化済)
//!
//! 閉路を検出した場合panicを起こす

use std::collections::VecDeque;

use weaverail_model::{error::ModelError, result_weft::WeftTempObj};

/// 最適化済みノードをトポロジカル順の添字列に整列する。
/// 計算量は O(|V|+|E|)
/// アルゴリズムは Kahn (1962) のトポロジカルソートを使用
/// 閉路を検出した場合は [`ModelError::DiagramGraphClosedPath`] を返す。
pub fn sort_node(tmp_obj: &WeftTempObj) -> Result<Vec<usize>, ModelError> {
    let count = tmp_obj.nodes.len();
    let mut num_input: Vec<usize> = vec![0; count];
    for node in &tmp_obj.nodes {
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
        let node_index = *que.front().ok_or(ModelError::DiagramGraphError)?;
        que.pop_front();
        answer.push(node_index);
        for edge in &tmp_obj
            .nodes
            .get(node_index)
            .ok_or(ModelError::DiagramGraphError)?
            .edges
        {
            let index = edge.0.0;
            num_input[index] -= 1;
            if num_input[index] == 0 {
                que.push_back(index);
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
    use weaverail_model::{model::Time, result_weft::{NodeId, WeftNode}};

    fn node(id: usize, edges: &[usize]) -> WeftNode {
        WeftNode {
            node_id: NodeId::new(id),
            edges: edges
                .iter()
                .map(|target| (NodeId::new(*target), Time::default()))
                .collect(),
            ..Default::default()
        }
    }

    #[test]
    fn sort_node_handles_empty_and_linear_graphs() {
        assert_eq!(
            sort_node(&WeftTempObj::default()).unwrap(),
            Vec::<usize>::new()
        );

        let graph = WeftTempObj {
            nodes: vec![node(0, &[1]), node(1, &[])],
            ..Default::default()
        };
        assert_eq!(sort_node(&graph).unwrap(), vec![0, 1]);
    }

    #[test]
    fn sort_node_reports_cycles() {
        let graph = WeftTempObj {
            nodes: vec![node(0, &[1]), node(1, &[0])],
            ..Default::default()
        };
        assert_eq!(sort_node(&graph), Err(ModelError::DiagramGraphClosedPath));
    }
}
