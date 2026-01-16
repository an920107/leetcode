/*
 * @lc app=leetcode id=1975 lang=rust
 *
 * [1975] Maximum Matrix Sum
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let nums: Vec<i32> = matrix.into_iter().flatten().collect();
        let nums_le_zero: Vec<&i32> = nums.iter().filter(|&n| *n <= 0).collect();
        let abs_sum = nums.iter().map(|&n| n.abs() as i64).sum::<i64>();

        if nums_le_zero.len() % 2 == 0 {
            abs_sum
        } else {
            abs_sum - (nums.iter().map(|&n| n.abs()).min().unwrap() * 2) as i64
        }
    }
}
// @lc code=end
