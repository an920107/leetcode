/*
 * @lc app=leetcode id=1390 lang=rust
 *
 * [1390] Four Divisors
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        nums.iter()
            .map(Solution::divisors_sum_if_exact_four)
            .sum::<i32>()
    }

    fn divisors_sum_if_exact_four(num: &i32) -> i32 {
        let lim = (*num as f32).sqrt() as i32;
        if lim * lim == *num {
            return 0;
        }

        let mut cnt = 1;
        let mut sum = 1 + num;
        for div in 2..=lim {
            if num % div == 0 {
                sum += div + num / div;
                cnt += 1;
            }
            if cnt > 2 {
                return 0;
            }
        }
        if cnt == 2 { sum } else { 0 }
    }
}
// @lc code=end
