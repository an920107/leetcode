pub struct Solution;

impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let mut is_all_zero = true;
        let mut current = 0;
        for &num in nums.iter() {
            if num != 0 {
                is_all_zero = false;
            }
            current ^= num;
        }
        if is_all_zero {
            0
        } else if current != 0 {
            nums.len() as i32
        } else {
            nums.len() as i32 - 1
        }
    }
}
