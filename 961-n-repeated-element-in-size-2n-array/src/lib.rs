/*
 * @lc app=leetcode id=961 lang=rust
 *
 * [961] N-Repeated Element in Size 2N Array
 */

pub struct Solution;

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((&mut HashSet::<i32>::new(), None), |(set, result), num| {
                if let Some(num) = set.take(num) {
                    (set, Some(num))
                } else {
                    set.insert(*num);
                    (set, result)
                }
            })
            .1
            .unwrap()
    }
}
// @lc code=end
