/*
 * @lc app=leetcode id=3314 lang=rust
 *
 * [3314] Construct the Minimum Bitwise Array I
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![-1; nums.len()];

        for (index, &num) in nums.iter().enumerate() {
            for m in 1..num {
                if m | (m + 1) == num {
                    ans[index] = m;
                    break;
                }
            }
        }

        ans
    }
}
// @lc code=end

