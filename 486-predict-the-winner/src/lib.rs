pub struct Solution;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut memo = vec![vec![None; n]; n];
        Self::max_diff(&nums, &mut memo, 0, n - 1) >= 0
    }

    fn max_diff(nums: &[i32], memo: &mut Vec<Vec<Option<i32>>>, l: usize, r: usize) -> i32 {
        if let Some(result) = memo[l][r] {
            return result;
        }

        if l == r {
            let result = nums[l];
            memo[l][r] = Some(result);
            return result;
        }

        let take_left = nums[l] - Self::max_diff(nums, memo, l + 1, r);
        let take_right = nums[r] - Self::max_diff(nums, memo, l, r - 1);
        let result = take_left.max(take_right);
        memo[l][r] = Some(result);
        result
    }
}
