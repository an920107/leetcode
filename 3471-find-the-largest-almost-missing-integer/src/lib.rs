pub struct Solution;

use std::collections::HashSet;

const MAX_NUM: i32 = 50;

impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let mut counts: Vec<i32> = vec![0; MAX_NUM as usize + 1];

        for window in nums.windows(k as usize) {
            for num in HashSet::<i32>::from_iter(window.iter().copied()) {
                counts[num as usize] += 1;
            }
        }

        counts
            .iter()
            .enumerate()
            .filter_map(|(num, &count)| if count == 1 { Some(num as i32) } else { None })
            .max()
            .unwrap_or(-1)
    }
}
