pub struct Solution;

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        nums.iter()
            .map(|num| {
                num.to_string()
                    .bytes()
                    .map(|c| (c - b'0') as i32)
                    .sum::<i32>()
            })
            .min()
            .unwrap()
    }
}
