pub struct Solution;

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        (nums.iter().max().unwrap() - nums.iter().min().unwrap()) as i64 * k as i64
    }
}
