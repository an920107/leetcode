/*
 * @lc app=leetcode id=3650 lang=rust
 *
 * [3650] Minimum Cost Path with Edge Reversals
 */

pub struct Solution;

// @lc code=start
use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn min_cost(n: i32, raw_edges: Vec<Vec<i32>>) -> i32 {
        let mut edges: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n as usize];
        for raw_edge in raw_edges.iter() {
            let u = raw_edge[0] as usize;
            let v = raw_edge[1] as usize;
            let w = raw_edge[2];
            edges[u].push((v, w));
            edges[v].push((u, w * 2));
        }

        let mut min_heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::from([Reverse((0, 0))]);
        let mut distances: Vec<i32> = vec![i32::MAX; n as usize];
        distances[0] = 0;

        while let Some(Reverse((dist, start))) = min_heap.pop() {
            for (end, weight) in edges[start].iter() {
                let new_dist = dist + weight;
                if new_dist < distances[*end] {
                    distances[*end] = new_dist;
                    min_heap.push(Reverse((new_dist, *end)));
                }
            }
        }

        let result = distances[n as usize - 1];
        if result == i32::MAX { -1 } else { result }
    }
}
// @lc code=end
