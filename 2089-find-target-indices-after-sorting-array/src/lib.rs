pub struct Solution;

impl Solution {
    pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.sort();
        nums.into_iter()
            .enumerate()
            .filter(|(_, num)| *num == target)
            .map(|(index, _)| index as i32)
            .collect()
    }
}
