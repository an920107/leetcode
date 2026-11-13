pub struct Solution;

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut current = if nums.len() >= 2 {
            if nums[0] != nums[1] { 2 } else { 1 }
        } else {
            1
        };
        let mut direction = if nums.len() >= 2 {
            if nums[0] < nums[1] {
                1
            } else if nums[0] > nums[1] {
                -1
            } else {
                0
            }
        } else {
            0
        };
        let mut result = current;

        for window in nums.windows(2).skip(1) {
            if window[0] < window[1] {
                if direction == 1 {
                    current += 1;
                } else {
                    direction = 1;
                    current = 2;
                }
            } else if window[0] > window[1] {
                if direction == -1 {
                    current += 1;
                } else {
                    direction = -1;
                    current = 2;
                }
            } else {
                direction = 0;
                current = 1;
            }
            result = result.max(current);
        }

        result
    }
}
