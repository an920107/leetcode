use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; 26];
        for i in 0..cost.len() {
            let u = original[i] as usize - 'a' as usize;
            let v = changed[i] as usize - 'a' as usize;
            let c = cost[i];
            graph[u].push((v, c));
        }

        let mut result = 0i64;

        let mut has_counted_distance: Vec<bool> = vec![false; 26];
        let mut distances: Vec<Vec<i32>> = vec![vec![i32::MAX; 26]; 26];
        for c in 0..26 {
            distances[c][c] = 0;
        }

        for (u, v) in source.bytes().zip(target.bytes()) {
            let u = u as usize - 'a' as usize;
            let v = v as usize - 'a' as usize;

            if !has_counted_distance[u] {
                // dijkstra
                let mut min_heap: BinaryHeap<Reverse<(i32, usize)>> =
                    BinaryHeap::from([Reverse((0, u))]);
                while let Some(Reverse((dist, node))) = min_heap.pop() {
                    for &(neighbor, cost) in graph[node].iter() {
                        let new_dist = dist + cost;
                        if new_dist < distances[u][neighbor] {
                            distances[u][neighbor] = new_dist;
                            min_heap.push(Reverse((new_dist, neighbor)));
                        }
                    }
                }
                has_counted_distance[u] = true;
            }

            if distances[u][v] == i32::MAX {
                return -1;
            }
            result += distances[u][v] as i64;
        }

        result
    }
}
