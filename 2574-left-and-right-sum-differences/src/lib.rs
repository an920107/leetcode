pub struct Solution;

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        let left_sum: Vec<i32> = std::iter::once(0)
            .chain(nums[..(n - 1)].iter().scan(0, |state, &num| {
                *state += num;
                Some(*state)
            }))
            .collect();
        let right_sum: Vec<i32> = std::iter::once(0)
            .chain(nums[1..].iter().rev().scan(0, |state, &num| {
                *state += num;
                Some(*state)
            }))
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();

        left_sum
            .into_iter()
            .zip(right_sum.into_iter())
            .map(|(a, b)| a.abs_diff(b) as i32)
            .collect()
    }
}
