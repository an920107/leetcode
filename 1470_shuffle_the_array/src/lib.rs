pub struct Solution;

impl Solution {
    pub fn shuffle(mut nums: Vec<i32>, _: i32) -> Vec<i32> {
        let right = nums.split_off(nums.len() / 2);
        nums.iter()
            .zip(right)
            .flat_map(|(&x, y)| vec![x, y])
            .collect()
    }
}
