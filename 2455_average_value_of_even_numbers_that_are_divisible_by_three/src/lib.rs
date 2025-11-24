pub struct Solution;

impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let filtered: Vec<i32> = nums.into_iter().filter(|num| *num % 6 == 0).collect();
        let sum: i32 = filtered.iter().sum();
        let len = filtered.len() as i32;
        if len == 0 { len } else { sum / len }
    }
}
