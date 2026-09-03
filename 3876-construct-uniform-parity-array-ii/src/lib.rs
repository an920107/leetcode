pub struct Solution;

impl Solution {
    pub fn uniform_array(nums: Vec<i32>) -> bool {
        let is_all_even = nums.iter().all(|num| *num % 2 == 0);
        let is_minimum_odd = nums.iter().min().unwrap() % 2 == 1;
        is_all_even || is_minimum_odd
    }
}
