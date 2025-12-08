pub struct Solution;

impl Solution {
    pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        nums.iter().filter(|&&num| num % 2 == 0).count() >= 2
    }
}
