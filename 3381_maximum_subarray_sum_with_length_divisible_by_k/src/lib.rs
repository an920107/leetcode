pub struct Solution;

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut prefixs: Vec<i64> = vec![0; nums.len() + 1];
        for (index, &num) in nums.iter().enumerate() {
            prefixs[index + 1] = prefixs[index] + num as i64;
        }

        let mut result = std::i64::MIN;
        for r in 0..k {
            let mut current_min = prefixs[r as usize];
            let mut index = (r + k) as usize;
            while index < prefixs.len() {
                let prefix = prefixs[index];
                result = result.max(prefix - current_min);
                current_min = current_min.min(prefix);
                index += k as usize;
            }
        }

        result
    }
}
