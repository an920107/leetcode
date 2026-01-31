pub struct Solution;

impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let k = k as usize;
        let mut result = i32::MAX;
        for i in 0..(nums.len() - k + 1) {
            let min = nums[i];
            let max = nums[i + k - 1];
            result = result.min(max - min);
        }
        result
    }
}
