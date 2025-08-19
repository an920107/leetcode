pub struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.iter().filter(|&num| num % 3 != 0).count() as i32
    }
}
