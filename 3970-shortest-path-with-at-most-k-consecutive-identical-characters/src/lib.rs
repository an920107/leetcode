pub struct Solution;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

impl Solution {
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        let labels = labels.as_bytes();
        let n = n as usize;
        let k = k as usize;

        let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
        for edge in &edges {
            let &[u, v, w] = edge.as_slice().try_into().unwrap();
            graph[u as usize].push((v as usize, w));
        }

        let mut visited: HashSet<(usize, usize, i32)> = HashSet::new();
        let mut min_heap: BinaryHeap<Reverse<HeapParams>> = BinaryHeap::new();
        min_heap.push(Reverse(HeapParams {
            total_weight: 0,
            consecutive_identical_count: 1,
            node: 0,
        }));

        while let Some(Reverse(current)) = min_heap.pop() {
            if current.node == n - 1 {
                return current.total_weight;
            }

            let current_lebel = labels[current.node];
            for &(neighbor, delta_weight) in &graph[current.node] {
                let neighbor_label = labels[neighbor];

                if (current.consecutive_identical_count < k || current_lebel != neighbor_label)
                    && !visited.contains(&(current.node, neighbor, delta_weight))
                {
                    visited.insert((current.node, neighbor, delta_weight));
                    min_heap.push(Reverse(HeapParams {
                        total_weight: current.total_weight + delta_weight,
                        consecutive_identical_count: if current_lebel == neighbor_label {
                            current.consecutive_identical_count + 1
                        } else {
                            1
                        },
                        node: neighbor,
                    }));
                }
            }
        }

        -1
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct HeapParams {
    total_weight: i32,
    consecutive_identical_count: usize,
    node: usize,
}
