/*
 * @lc app=leetcode id=2975 lang=rust
 *
 * [2975] Maximum Square Area by Removing Fences From a Field
 */

pub struct Solution;

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn maximize_square_area(
        m: i32,
        n: i32,
        mut h_fences: Vec<i32>,
        mut v_fences: Vec<i32>,
    ) -> i32 {
        let mut h_set: HashSet<i32> = HashSet::from_iter([m - 1]);
        let mut w_set: HashSet<i32> = HashSet::from_iter([n - 1]);

        h_fences.sort();
        v_fences.sort();

        for (i, fence_a) in h_fences.iter().enumerate() {
            h_set.insert(*fence_a - 1);
            for fence_b in h_fences[i + 1..h_fences.len()].iter() {
                h_set.insert(*fence_b - fence_a);
            }
            h_set.insert(m - fence_a);
        }

        for (i, fence_a) in v_fences.iter().enumerate() {
            w_set.insert(*fence_a - 1);
            for fence_b in v_fences[i + 1..v_fences.len()].iter() {
                w_set.insert(*fence_b - fence_a);
            }
            w_set.insert(n - fence_a);
        }

        let mut result = -1i64;
        for &h in h_set.iter() {
            if w_set.contains(&h) {
                result = result.max(h as i64 * h as i64);
            }
        }

        (result % 1_000_000_007) as i32
    }
}
// @lc code=end
