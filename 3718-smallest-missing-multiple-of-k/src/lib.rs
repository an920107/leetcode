pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let nums_set: HashSet<i32> = HashSet::from_iter(nums.into_iter());
        (1..)
            .map(|m| k * m)
            .skip_while(|p| nums_set.contains(p))
            .next()
            .unwrap()
    }
}
