pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = (k as usize) % n;

        if k == 0 {
            return;
        }

        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}
