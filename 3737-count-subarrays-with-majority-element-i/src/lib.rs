pub struct Solution;

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        let mut prefix: Vec<i32> = vec![0; nums.len() + 1];

        for (index, &num) in nums.iter().enumerate() {
            prefix[index + 1] = prefix[index];
            if num == target {
                prefix[index + 1] += 1;
            }
        }

        let mut result = 0;

        for i in 0..nums.len() {
            for j in i..nums.len() {
                let len = (j - i + 1) as i32;
                let target_count = prefix[j + 1] - prefix[i];
                if target_count != 0 && len / target_count < 2 {
                    result += 1;
                }
            }
        }

        result
    }
}
