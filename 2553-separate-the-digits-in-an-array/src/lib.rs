pub struct Solution;

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .map(|num| {
                num.to_string()
                    .bytes()
                    .map(|c| (c - b'0') as i32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .concat()
    }
}
