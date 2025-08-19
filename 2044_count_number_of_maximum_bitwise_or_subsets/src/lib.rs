pub struct Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let max_or_sum = nums.iter().fold(0, |acc, &num| acc | num);

        let mut result = 0;
        let mut stack: Vec<DFSParams> = nums
            .iter()
            .enumerate()
            .map(|(index, &num)| DFSParams { index, or_sum: num })
            .collect();

        while !stack.is_empty() {
            let current = stack.pop().unwrap();

            if current.or_sum == max_or_sum {
                result += 1 << (nums.len() - current.index - 1);
                continue;
            }

            for i in (current.index + 1)..nums.len() {
                stack.push(DFSParams {
                    index: i,
                    or_sum: current.or_sum | nums[i],
                });
            }
        }

        result
    }
}

pub struct DFSParams {
    index: usize,
    or_sum: i32,
}
