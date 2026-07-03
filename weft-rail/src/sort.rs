//! # sort
//! 
//! Kahn (1962) のトポロジカルソートを使用し、有向グラフを一次元に整列するモジュール
//! 
//! 閉路を検出した場合panicを起こす

use std::collections::VecDeque;

use weaverail_model::result_weft::{NodeId, WeftNode};

/// ノードを整列する関数
/// 計算量は O(|V|+|E|)
/// アルゴリズムは Kahn (1962) のトポロジカルソートを使用
pub fn sort_node<'a>(nodes: &'a Vec<&'a WeftNode>) -> Vec<&'a WeftNode> {
    let count = nodes.len();
    // 入次数
    let mut num_input: Vec<usize> = vec![0; count];

    for node in nodes {
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

    let mut answer = Vec::with_capacity(count);
    while !que.is_empty() {
        let node = {
            let node_index = que.front().unwrap();
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
        panic!("閉路！！！");
    }

    answer
}
