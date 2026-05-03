pub struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let or_sum = nums.iter().fold(0, |acc, &num| acc | num);
        or_sum << (nums.len() - 1)
    }
}
