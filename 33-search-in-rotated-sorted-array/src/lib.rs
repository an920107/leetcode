pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let last_num = nums[n - 1];
        let rotated_count = nums.partition_point(|&num| num > last_num);

        let mut left = 0;
        let mut right = n;
        while left < right {
            let mid = (left + right) / 2;
            let mapped_mid = (mid + rotated_count) % n;

            if nums[mapped_mid] == target {
                return mapped_mid as i32;
            } else if nums[mapped_mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        -1
    }
}
