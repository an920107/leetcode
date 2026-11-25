pub struct Solution;

use std::collections::HashMap;

impl Solution {
    const BASE: i64 = 100_003;
    const MOD: i64 = 1_000_000_007;

    pub fn smallest_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut left = 1;
        let mut right = nums.len() + 1;

        while left < right {
            let mid = (left + right) / 2;
            if Self::is_possible(&nums, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left as i32
    }

    fn is_possible(nums: &[i32], window_size: usize) -> bool {
        let mut hash_val = 0i64;
        let mut highest_pow = 1i64;

        for (index, &num) in nums[..window_size].iter().enumerate() {
            hash_val = (hash_val * Self::BASE + num as i64) % Self::MOD;
            if index > 0 {
                highest_pow = (highest_pow * Self::BASE) % Self::MOD;
            }
        }

        let mut freq: HashMap<i64, usize> = HashMap::new();
        freq.insert(hash_val, 1);

        for window in nums.windows(window_size + 1) {
            let to_remove = window[0] as i64;
            let to_append = window[window.len() - 1] as i64;

            let removing_val = (to_remove * highest_pow) % Self::MOD;
            hash_val = (hash_val - removing_val + Self::MOD) % Self::MOD;
            hash_val = (hash_val * Self::BASE + to_append) % Self::MOD;

            *freq.entry(hash_val).or_insert(0) += 1;
        }

        freq.iter().any(|(_, &count)| count == 1)
    }
}
