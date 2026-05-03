/*
 * @lc app=leetcode id=66 lang=rust
 *
 * [66] Plus One
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        digits.reverse();

        let mut carry = 1;
        for digit in digits.iter_mut() {
            *digit += carry;
            carry = *digit / 10;
            *digit %= 10;
        }

        if carry > 0 {
            digits.push(carry);
        }

        digits.reverse();
        digits
    }
}
// @lc code=end
