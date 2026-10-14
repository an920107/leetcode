pub struct Solution;

impl Solution {
    pub fn max_alternating_sum(mut nums: Vec<i32>) -> i64 {
        let n = nums.len();
        nums.iter_mut().for_each(|num| *num *= *num);
        nums.sort();
        nums[(n / 2)..].iter().map(|num| *num as i64).sum::<i64>()
            - nums[..(n / 2)].iter().map(|num| *num as i64).sum::<i64>()
    }
}
