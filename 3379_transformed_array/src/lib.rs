pub struct Solution;

impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        nums.iter()
            .enumerate()
            .map(|(index, &num)| nums[(n as i32 + (index as i32 + num) % n as i32) as usize % n])
            .collect()
    }
}
