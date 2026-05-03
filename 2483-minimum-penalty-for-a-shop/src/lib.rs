/*
 * @lc app=leetcode id=2483 lang=rust
 *
 * [2483] Minimum Penalty for a Shop
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let n = customers.len();
        let mut penalty = customers.bytes().filter(|&c| c == 'Y' as u8).count() as i32;

        let mut result = 0usize;
        let mut min_penalty = penalty;
        for time in 1..=n {
            penalty += if customers.as_bytes()[time - 1] == 'Y' as u8 {
                -1
            } else {
                1
            };

            if penalty < min_penalty {
                min_penalty = penalty;
                result = time;
            }
        }

        result as i32
    }
}
// @lc code=end
