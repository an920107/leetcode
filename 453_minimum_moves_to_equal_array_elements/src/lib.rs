pub struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let min = nums.iter().min().unwrap();
        sum - *min * nums.len() as i32
    }
}
