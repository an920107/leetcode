pub struct Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut last_one_index: i32 = -1_000_000;

        for (index, &num) in nums.iter().enumerate() {
            if num == 1 {
                let distance = index as i32 - last_one_index - 1;
                if distance < k {
                    return false;
                }
                last_one_index = index as i32;
            }
        }

        true
    }
}
