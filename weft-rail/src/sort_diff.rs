use std::collections::VecDeque;

use crate::make_node_diff::WeftTempObj;

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
