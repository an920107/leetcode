pub struct Solution;

impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = 0;

        nums.sort();

        for i in 0..(n / 2) {
            result = result.max(nums[i] + nums[n - i - 1]);
        }

        result
    }
}
