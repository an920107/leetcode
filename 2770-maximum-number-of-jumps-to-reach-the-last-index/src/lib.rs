pub struct Solution;

impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        let mut memo = vec![None; nums.len()];
        Self::recursion(&nums, target, &mut memo, 0).unwrap_or(-1)
    }

    fn recursion(
        nums: &[i32],
        target: i32,
        memo: &mut [Option<Option<i32>>],
        start: usize,
    ) -> Option<i32> {
        if start >= nums.len() - 1 {
            return Some(0);
        }

        if let Some(result) = memo[start] {
            return result;
        }

        let mut max_jump: Option<i32> = None;
        for i in (start + 1)..nums.len() {
            if nums[start].abs_diff(nums[i]) as i32 > target {
                continue;
            }
            max_jump = match (max_jump, Self::recursion(nums, target, memo, i)) {
                (None, None) => None,
                (None, Some(v)) | (Some(v), None) => Some(v),
                (Some(v1), Some(v2)) => Some(v1.max(v2)),
            }
        }

        let result = max_jump.map(|v| v + 1);
        memo[start] = Some(result);
        result
    }
}
