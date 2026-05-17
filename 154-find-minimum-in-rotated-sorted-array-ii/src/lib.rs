pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let k = nums.last().copied().unwrap();
        let mut start = 0;
        while start < nums.len() - 1 && nums[start] == k {
            start += 1;
        }
        let i = nums[start..].partition_point(|&num| num > k);
        nums[i + start]
    }
}
