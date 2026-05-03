/*
 * @lc app=leetcode id=3075 lang=rust
 *
 * [3075] Maximize Happiness of Selected Children
 */

pub struct Solution;

// @lc code=start
use std::collections::BinaryHeap;

impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let mut heap: BinaryHeap<i32> = BinaryHeap::from_iter(happiness);

        let mut result = 0i64;
        let mut round = 0;
        while round < k {
            let top = heap.pop().unwrap();
            result += (top - round).max(0) as i64;
            round += 1;
        }

        result
    }
}
// @lc code=end
