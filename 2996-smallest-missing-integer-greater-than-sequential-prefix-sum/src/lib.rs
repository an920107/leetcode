pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut last_num = nums[0] - 1;
        for &num in nums.iter() {
            if num - last_num == 1 {
                sum += num;
            } else {
                break;
            }
            last_num = num;
        }

        let nums_set: HashSet<i32> = HashSet::from_iter(nums.into_iter());
        while nums_set.contains(&sum) {
            sum += 1;
        }
        return sum;
    }
}
