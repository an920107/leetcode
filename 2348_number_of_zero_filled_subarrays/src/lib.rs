pub struct Solution;

impl Solution {
    pub fn zero_filled_subarray(mut nums: Vec<i32>) -> i64 {
        let mut result = 0i64;

        let mut continuous_count = 0i64;
        nums.push(1);
        for num in nums {
            if num == 0 {
                continuous_count += 1;
            } else {
                result += (1 + continuous_count) * continuous_count / 2;
                continuous_count = 0;
            }
        }

        result
    }
}
