pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        let n = online.len();

        let mut left = i64::MAX;
        let mut right = 0;

        let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
        for edge in edges.into_iter() {
            let (u, v, cost) = (edge[0] as usize, edge[1] as usize, edge[2] as i64);
            if !(online[u] && online[v]) {
                continue;
            }
            graph[u].push((v, cost));
            left = left.min(cost);
            right = right.max(cost);
        }

        if !Self::check(&graph, n, left, k) {
            return -1;
        }

        right += 1;
        while left < right {
            let mid = (left + right) / 2;
            if Self::check(&graph, n, mid, k) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left as i32 - 1
    }

    fn check(graph: &Vec<Vec<(usize, i64)>>, n: usize, min_cost_limit: i64, k: i64) -> bool {
        let mut min_distances: Vec<i64> = vec![i64::MAX; n];
        let mut min_heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        min_distances[0] = 0;
        min_heap.push(Reverse((0, 0)));

        while let Some(Reverse((current_distance, current_node))) = min_heap.pop() {
            if current_distance > k {
                return false;
            }
            if current_node == n - 1 {
                return true;
            }
            if current_distance > min_distances[current_node] {
                continue;
            }

            for &(neighbor, cost) in graph[current_node].iter() {
                if cost < min_cost_limit {
                    continue;
                }
                let new_distance = current_distance + cost;
                if new_distance < min_distances[neighbor] {
                    min_distances[neighbor] = new_distance;
                    min_heap.push(Reverse((new_distance, neighbor)));
                }
            }
        }

        false
    }
}
