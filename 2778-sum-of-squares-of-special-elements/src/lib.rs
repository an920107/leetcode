pub struct Solution;

impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .filter(|(index, _)| nums.len() % (index + 1) == 0)
            .map(|(_, num)| num * num)
            .sum()
    }
}
