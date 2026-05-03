/*
 * @lc app=leetcode id=1351 lang=rust
 *
 * [1351] Count Negative Numbers in a Sorted Matrix
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut col = grid.last().unwrap().partition_point(|&num| num >= 0);
        let mut result = 0;

        for row in (0..m).rev() {
            while col < n && grid[row][col] >= 0 {
                col += 1;
            }
            result += n - col;
        }

        result as i32
    }
}
// @lc code=end
