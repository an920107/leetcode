pub struct Solution;

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        nums.iter()
            .scan(0, |state, num| {
                *state = ((*state << 1) + num) % 5;
                Some(*state % 5 == 0)
            })
            .collect()
    }
}
