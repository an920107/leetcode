pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let k = nums.last().copied().unwrap();
        let i = nums.partition_point(|&num| num > k);
        nums[i]
    }
}
