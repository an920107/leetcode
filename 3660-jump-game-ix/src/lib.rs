pub struct Solution;

impl Solution {
    pub fn max_value(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result: Vec<i32> = vec![0; n];

        let mut left_max: Vec<i32> = vec![nums[0]; n];
        for i in 1..n {
            left_max[i] = left_max[i - 1].max(nums[i]);
        }

        let mut right_min: Vec<i32> = vec![nums[n - 1]; n];
        for i in (0..(n - 1)).rev() {
            right_min[i] = right_min[i + 1].min(nums[i]);
        }

        result[n - 1] = left_max[n - 1];
        for i in (0..(n - 1)).rev() {
            if left_max[i] > right_min[i + 1] {
                result[i] = result[i + 1];
            } else {
                result[i] = left_max[i];
            }
        }

        result
    }
}
