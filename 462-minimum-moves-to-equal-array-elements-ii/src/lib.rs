pub struct Solution;

impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort();
        let mid = if n % 2 == 0 {
            ((nums[n / 2 - 1] as i64 + nums[n / 2] as i64) / 2) as i32
        } else {
            nums[n / 2]
        };
        nums.into_iter()
            .map(|num| num.abs_diff(mid) as i32)
            .sum::<i32>()
    }
}
