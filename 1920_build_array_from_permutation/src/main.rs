fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().map(|&val| nums[val as usize]).collect()
    }
}
