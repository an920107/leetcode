pub struct Solution;

impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();

        let mut max_prefix = vec![i32::MIN; n + 1];
        for (index, &num) in nums.iter().enumerate() {
            max_prefix[index + 1] = max_prefix[index].max(num);
        }

        let mut min_prefix = vec![i32::MAX; n + 1];
        for (index, &num) in nums.iter().enumerate().rev() {
            min_prefix[index] = min_prefix[index + 1].min(num);
        }

        (0..n)
            .filter(|&i| max_prefix[i + 1] - min_prefix[i] <= k)
            .next()
            .map(|i| i as i32)
            .unwrap_or(-1)
    }
}
