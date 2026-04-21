pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn minimum_hamming_distance(
        source: Vec<i32>,
        target: Vec<i32>,
        allowed_swaps: Vec<Vec<i32>>,
    ) -> i32 {
        let n = source.len();
        let mut us = UnionSet::new(n);
        for pair in allowed_swaps.into_iter() {
            us.union(pair[0] as usize, pair[1] as usize);
        }
        let mut source_map: Vec<HashMap<i32, i32>> = vec![HashMap::new(); n];
        let mut target_map: Vec<HashMap<i32, i32>> = vec![HashMap::new(); n];
        for (i, (s, t)) in source.into_iter().zip(target.into_iter()).enumerate() {
            let parent = us.find(i);
            target_map[parent]
                .entry(s)
                .and_modify(|v| *v += 1)
                .or_insert(1);
            source_map[parent]
                .entry(t)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
        let mut result = 0;
        for i in 0..n {
            for (k, v) in source_map[i].iter() {
                let diff = v - target_map[i].get(k).copied().unwrap_or(0);
                if diff > 0 {
                    result += diff;
                }
            }
        }

        result
    }
}

struct UnionSet {
    link_parent: Vec<usize>,
}

impl UnionSet {
    fn new(size: usize) -> Self {
        Self {
            link_parent: Vec::from_iter(0..size),
        }
    }

    fn union(&mut self, key0: usize, key1: usize) {
        let l = self.find(key0);
        let r = self.find(key1);
        self.link_parent[l] = r;
    }

    fn find(&self, key: usize) -> usize {
        if self.link_parent[key] == key {
            key
        } else {
            Self::find(&self, self.link_parent[key])
        }
    }
}
