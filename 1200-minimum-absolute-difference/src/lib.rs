/*
 * @lc app=leetcode id=1200 lang=rust
 *
 * [1200] Minimum Absolute Difference
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort();

        let mut min_diff = i32::MAX;
        let mut result: Vec<Vec<i32>> = vec![];

        for i in 1..arr.len() {
            let diff = arr[i - 1].abs_diff(arr[i]) as i32;

            if diff < min_diff {
                result.clear();
                min_diff = diff;
            }

            if diff <= min_diff {
                result.push(vec![arr[i - 1], arr[i]]);
            }
        }

        result
    }
}
// @lc code=end
