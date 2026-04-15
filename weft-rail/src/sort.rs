use std::collections::{HashMap, VecDeque};

use crate::{NodeId, WeftNode};

/// ノードを整列する関数
/// 計算量は O(|V|+|E|)
/// アルゴリズムは Kahn (1962) のトポロジカルソートを使用
pub(crate) fn sort_node(nodes: &HashMap<NodeId, WeftNode>) -> Vec<&WeftNode> {
    let count = nodes.len();
    // 入次数
    let mut num_input: Vec<usize> = vec![0; count];

    for node in nodes.values() {
        for edge in &node.edges {
            num_input[edge.0.0] += 1;
        }
    }

    let mut que = VecDeque::new();
    for i in 0..count {
        if num_input[i] == 0 {
            que.push_back(i);
        }
    }

    let mut answer = Vec::new();
    while !que.is_empty() {
        let node = {
            let node_index = que.front().unwrap();
            let node_id = NodeId::new(*node_index);
            nodes.get(&node_id).unwrap()
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
        panic!("閉路！！！");
    }

    answer
}
