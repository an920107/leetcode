pub struct Solution;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    i64,
};

use itertools::izip;

type Graph = HashMap<String, Vec<(String, i64)>>;
type Distance = HashMap<String, i64>;

const INF: i64 = i64::MAX;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<String>,
        changed: Vec<String>,
        cost: Vec<i32>,
    ) -> i64 {
        let mut graph: Graph = HashMap::new();
        for (u, v, c) in izip!(&original, &changed, &cost) {
            graph
                .entry(u.clone())
                .and_modify(|m| m.push((v.clone(), *c as i64)))
                .or_insert(vec![(v.clone(), *c as i64)]);
        }

        let mut distances: HashMap<String, Distance> = HashMap::new();
        let mut memo: Vec<Option<Option<i64>>> = vec![None; source.len()];

        Solution::recursion(0, &source, &target, &graph, &mut distances, &mut memo).unwrap_or(-1)
    }

    fn recursion(
        substr_begin_index: usize,
        source: &str,
        target: &str,
        graph: &Graph,
        distances: &mut HashMap<String, Distance>,
        memo: &mut Vec<Option<Option<i64>>>,
    ) -> Option<i64> {
        if substr_begin_index >= source.len() {
            return Some(0);
        }

        if let Some(result) = memo[substr_begin_index] {
            return result;
        }

        let mut result: Option<i64> = None;

        for u in graph.keys() {
            if !source[substr_begin_index..].starts_with(u) {
                continue;
            }

            let substr_end_index = substr_begin_index + u.len();
            let v = &target[substr_begin_index..substr_end_index];
            let cost = Solution::dijkstra(u, v, graph, distances);

            if let Some(cost) = cost
                && let Some(next_result) =
                    Solution::recursion(substr_end_index, source, target, graph, distances, memo)
            {
                result = Some(result.unwrap_or(INF).min(cost + next_result));
            }
        }

        for (offset, (source_c, target_c)) in source
            .bytes()
            .zip(target.bytes())
            .skip(substr_begin_index)
            .enumerate()
        {
            if source_c != target_c {
                break;
            }

            let new_substr_begin_index = substr_begin_index + offset + 1;
            if let Some(next_result) = Solution::recursion(
                new_substr_begin_index,
                source,
                target,
                graph,
                distances,
                memo,
            ) {
                result = Some(result.unwrap_or(INF).min(next_result));
            }
        }

        memo[substr_begin_index] = Some(result);
        result
    }

    fn dijkstra(
        u: &str,
        v: &str,
        graph: &Graph,
        distances: &mut HashMap<String, Distance>,
    ) -> Option<i64> {
        if let Some(cache) = distances.get(u) {
            return cache.get(v).copied();
        }

        distances.insert(u.to_string(), HashMap::from([(u.to_string(), 0)]));
        let distance = distances.get_mut(u).unwrap();

        let mut min_heap: BinaryHeap<Reverse<(i64, String)>> =
            BinaryHeap::from([Reverse((0, u.to_string()))]);

        while let Some(Reverse((dist, node))) = min_heap.pop() {
            if let Some(neighbors) = graph.get(&node) {
                for (neighbor, cost) in neighbors {
                    let new_dist = dist + cost;
                    if new_dist < *distance.get(neighbor).unwrap_or(&INF) {
                        min_heap.push(Reverse((new_dist, neighbor.clone())));
                        distance.insert(neighbor.clone(), new_dist);
                    }
                }
            }
        }

        distance.get(v).copied()
    }
}
