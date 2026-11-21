pub struct Solution;

impl Solution {
    pub fn max_selected_elements(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by(|a, b| b.cmp(a));

        let mut memo = vec![vec![None; 2]; nums.len() + 1];
        let mut result = 1;

        for index in 0..nums.len() {
            let with_op = Self::recursion(&mut memo, &nums, index, true);
            let without_op = Self::recursion(&mut memo, &nums, index, false);
            result = result.max(with_op).max(without_op);
        }
        result as i32
    }

    fn recursion(
        memo: &mut Vec<Vec<Option<usize>>>,
        nums: &[i32],
        index: usize,
        op: bool,
    ) -> usize {
        if index + 1 >= nums.len() {
            return 1;
        }

        let op_index = usize::from(op);

        if let Some(result) = memo[index][op_index] {
            return result;
        }

        let num = nums[index] + i32::from(op);
        let next_num = nums[index + 1];
        let diff = num - next_num;

        let with_op = Self::recursion(memo, nums, index + 1, true);
        let without_op = Self::recursion(memo, nums, index + 1, false);

        let result = match diff {
            0 => without_op,
            1 => without_op + 1,
            2 => with_op + 1,
            _ => 1,
        };

        memo[index][op_index] = Some(result);
        result
    }
}
