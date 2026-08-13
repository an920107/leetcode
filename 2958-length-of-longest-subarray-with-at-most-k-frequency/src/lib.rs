pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut frequency: HashMap<i32, i32> = HashMap::new();
        let mut left_index = 0;
        let mut right_index = 0;
        let mut max_length = 1;
        *frequency.entry(nums[0]).or_default() += 1;

        while right_index < nums.len() - 1 {
            right_index += 1;
            *frequency.entry(nums[right_index]).or_default() += 1;
            while frequency[&nums[right_index]] > k {
                *frequency.entry(nums[left_index]).or_default() -= 1;
                left_index += 1;
            }
            max_length = max_length.max(right_index - left_index + 1);
        }

        max_length as i32
    }
}
