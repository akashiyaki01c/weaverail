//! # sort_diff
//!
//! Kahn (1962) のトポロジカルソートを使用し、有向グラフを一次元に整列するモジュール (最適化済)
//!
//! 閉路を検出した場合panicを起こす

use std::collections::VecDeque;

use weaverail_model::result_weft::WeftTempObj;

/// ノードを整列する関数
/// 計算量は O(|V|+|E|)
/// アルゴリズムは Kahn (1962) のトポロジカルソートを使用
pub fn sort_node(tmp_obj: &WeftTempObj) -> Vec<usize> {
    let count = tmp_obj.nodes.len();
    let mut num_input: Vec<usize> = vec![0; count];
    for node in &tmp_obj.nodes {
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
        let node_index = *que.front().unwrap();
        que.pop_front();
        answer.push(node_index);
        for edge in &tmp_obj.nodes.get(node_index).unwrap().edges {
            let index = edge.0.0;
            num_input[index] -= 1;
            if num_input[index] == 0 {
                que.push_back(index);
            }
        }
    }

    if answer.len() != count {
        panic!("閉路！！！");
    }

    answer
}
