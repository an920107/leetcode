pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut prefixs = Vec::with_capacity(nums.len() + 1);
        prefixs.push(0);
        for &num in nums.iter() {
            prefixs.push(prefixs.last().unwrap() + num);
        }

        let mut suffixs = Vec::with_capacity(nums.len() + 1);
        suffixs.push(0);
        for &num in nums.iter().rev() {
            suffixs.push(suffixs.last().unwrap() + num);
        }

        let mut result = None;
        for (suffix_index, &suffix) in suffixs.iter().enumerate() {
            if suffix > x {
                break;
            }
            if let Ok(prefix_index) = prefixs.binary_search(&(x - suffix))
                && prefix_index + suffix_index <= nums.len()
            {
                result = Some(
                    result
                        .unwrap_or(i32::MAX)
                        .min((prefix_index + suffix_index) as i32),
                );
            }
        }

        result.unwrap_or(-1)
    }
}
