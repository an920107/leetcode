pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
        Solution::recursion((0, 0), (&nums1, &nums2), &mut memo).unwrap()
    }

    fn recursion(
        index: (usize, usize),
        nums: (&[i32], &[i32]),
        memo: &mut HashMap<(usize, usize), i32>,
    ) -> Option<i32> {
        if index.0 >= nums.0.len() || index.1 >= nums.1.len() {
            return None;
        }

        if let Some(result) = memo.get(&index) {
            return Some(*result);
        }

        let product = nums.0[index.0] * nums.1[index.1];

        let skip_0 = Solution::recursion((index.0 + 1, index.1), nums, memo);
        let skip_1 = Solution::recursion((index.0, index.1 + 1), nums, memo);
        let no_skip = Solution::recursion((index.0 + 1, index.1 + 1), nums, memo);

        let mut result = product;
        if let Some(s) = skip_0 {
            result = result.max(s);
        }
        if let Some(s) = skip_1 {
            result = result.max(s);
        }
        if let Some(s) = no_skip {
            result = result.max(s + product);
        }

        memo.insert(index, result);
        Some(result)
    }
}
