/*
 * @lc app=leetcode id=3315 lang=rust
 *
 * [3315] Construct the Minimum Bitwise Array II
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![-1; nums.len()];

        for (index, &num) in nums.iter().enumerate() {
            let mut num = num;
            let mut continuous_ones = 0;
            let mut tail = 0;

            while num & 1 > 0 {
                continuous_ones += 1;
                num >>= 1;
                tail = (tail << 1) + 1;
            }

            if continuous_ones == 0 {
                continue;
            }

            num <<= continuous_ones;
            num += tail >> 1;
            ans[index] = num;
        }

        ans
    }
}
// @lc code=end
