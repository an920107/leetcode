pub struct Solution;

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let valid_nums = [
            12, 23, 34, 45, 56, 67, 78, 89, 123, 234, 345, 456, 567, 678, 789, 1234, 2345, 3456,
            4567, 5678, 6789, 12345, 23456, 34567, 45678, 56789, 123456, 234567, 345678, 456789,
            1234567, 2345678, 3456789, 12345678, 23456789, 123456789,
        ];

        let start_index = valid_nums.partition_point(|n| *n < low);
        let end_index = valid_nums.partition_point(|n| *n <= high);

        if start_index >= end_index {
            vec![]
        } else {
            valid_nums[start_index..end_index].iter().copied().collect()
        }
    }
}
