/*
 * @lc app=leetcode id=1411 lang=rust
 *
 * [1411] Number of Ways to Paint N × 3 Grid
 */

pub struct Solution;

// @lc code=start
const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        Solution::recursion(6, 6, n)
    }

    fn recursion(xyx_count: i64, xyz_count: i64, layer: i32) -> i32 {
        if layer == 1 {
            return ((xyx_count + xyz_count) % MOD) as i32;
        }

        Solution::recursion(
            (xyx_count * 3 + xyz_count * 2) % MOD,
            (xyx_count * 2 + xyz_count * 2) % MOD,
            layer - 1,
        )
    }
}
// @lc code=end
