pub struct Solution;

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut max_num = i32::MIN;
        let mut max_num_index = 0;
        let mut min_num = i32::MAX;
        let mut min_num_index = 0;

        for (index, &num) in nums.iter().enumerate() {
            if num > max_num {
                max_num = num;
                max_num_index = index;
            }
            if num < min_num {
                min_num = num;
                min_num_index = index;
            }
        }

        let indices = (
            min_num_index.min(max_num_index),
            min_num_index.max(max_num_index),
        );

        ((indices.0 + 1) + (n - indices.1))
            .min((indices.0 + 1) + (indices.1 - indices.0))
            .min((n - indices.1) + (indices.1 - indices.0)) as i32
    }
}
