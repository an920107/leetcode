pub struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(mut nums: Vec<i32>) -> i32 {
        nums.push(0);
        let mut result = 0;

        let mut last_num = nums[0];
        let mut current_len = last_num;

        for &num in nums.iter().skip(1) {
            if num == 0 {
                if last_num == 1 {
                    result = result.max(current_len);
                    current_len = 0;
                }
            } else {
                current_len += 1;
            }

            last_num = num;
        }

        result
    }
}
