/*
 * @lc app=leetcode id=3507 lang=rust
 *
 * [3507] Minimum Pair Removal to Sort Array I
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut count = 0;
        while !nums.is_sorted() {
            let (sum, index) = Solution::minimum_pair(&nums);
            nums.remove(index);
            nums[index] = sum;
            count += 1;
        }
        count
    }

    fn minimum_pair(nums: &[i32]) -> (i32, usize) {
        let mut result_sum = i32::MAX;
        let mut result_index = 0usize;
        for i in 0..(nums.len() - 1) {
            let sum = nums[i] + nums[i + 1];
            if sum < result_sum {
                result_sum = sum;
                result_index = i;
            }
        }
        (result_sum, result_index)
    }
}
// @lc code=end
