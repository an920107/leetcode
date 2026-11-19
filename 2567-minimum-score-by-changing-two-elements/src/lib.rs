pub struct Solution;

use std::mem::swap;

impl Solution {
    pub fn minimize_sum(nums: Vec<i32>) -> i32 {
        let (mut min_1, mut min_2, mut min_3, mut max_1, mut max_2, mut max_3) =
            (i32::MAX, i32::MAX, i32::MAX, i32::MIN, i32::MIN, i32::MIN);

        for num in nums.into_iter() {
            min_3 = min_3.min(num);
            if min_3 < min_2 {
                swap(&mut min_3, &mut min_2);
            }
            if min_2 < min_1 {
                swap(&mut min_2, &mut min_1);
            }

            max_3 = max_3.max(num);
            if max_3 > max_2 {
                swap(&mut max_3, &mut max_2);
            }
            if max_2 > max_1 {
                swap(&mut max_2, &mut max_1);
            }
        }

        (max_1 - min_3).min(max_2 - min_2).min(max_3 - min_1)
    }
}
