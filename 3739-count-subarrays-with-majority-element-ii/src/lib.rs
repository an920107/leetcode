pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i64 {
        let mut prefix: Vec<i32> = vec![0; nums.len() + 1];

        for (index, &num) in nums.iter().enumerate() {
            prefix[index + 1] = prefix[index] + if num == target { 1 } else { -1 };
        }

        let min_of_prefix = *prefix.iter().min().unwrap();
        let max_of_prefix = *prefix.iter().max().unwrap();
        let mut prefix_count: HashMap<i32, i32> = HashMap::new();
        for &p in prefix.iter() {
            *prefix_count.entry(p - min_of_prefix + 1).or_default() += 1;
        }

        let mut fenwick_tree = FenwickTree::new((max_of_prefix - min_of_prefix + 1) as usize);
        for (&key, &value) in prefix_count.iter() {
            fenwick_tree.add(key as usize, value);
        }

        let mut result = 0;
        for (i, &p) in prefix[..(prefix.len() - 1)].iter().enumerate() {
            let sum_of_prefix = (prefix.len() - i) as i64;
            let moved_index = (p - min_of_prefix + 1) as usize;
            result += sum_of_prefix - fenwick_tree.query(moved_index);
            fenwick_tree.add(moved_index, -1);
        }

        result
    }
}

struct FenwickTree {
    tree: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
        }
    }

    fn lowbit(x: usize) -> usize {
        x & x.wrapping_neg()
    }

    fn add(&mut self, index: usize, delta: i32) {
        if index < self.tree.len() {
            self.tree[index] += delta as i64;
            self.add(index + Self::lowbit(index), delta);
        }
    }

    fn query(&self, mut index: usize) -> i64 {
        let mut sum = 0;
        while index > 0 {
            sum += self.tree[index];
            index -= Self::lowbit(index);
        }
        sum
    }
}
