pub struct Solution;

use std::mem::swap;

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let mut min_1 = i32::MAX;
        let mut min_2 = i32::MAX;

        for &num in nums[1..].iter() {
            min_2 = min_2.min(num);
            if min_2 < min_1 {
                swap(&mut min_1, &mut min_2);
            }
        }

        nums[0] + min_1 + min_2
    }
}
