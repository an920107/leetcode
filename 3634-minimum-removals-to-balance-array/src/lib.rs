pub struct Solution;

impl Solution {
    pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();

        let mut result = i32::MAX;

        for (index, &num) in nums.iter().enumerate() {
            let limit = num as i64 * k as i64;
            let partition_point = nums.partition_point(|&num| num as i64 <= limit);
            let right_rm_count = if partition_point < index {
                0
            } else {
                nums.len() - partition_point
            };
            let left_rm_count = index;
            result = result.min((right_rm_count + left_rm_count) as i32);
        }

        result
    }
}
