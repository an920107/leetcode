pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut index = 0;
        let mut stack: Vec<i32> = Vec::new();

        while index < nums.len() {
            let num = nums[index];

            if let Some(&maximum) = stack.last() {
                if num > maximum {
                    stack.push(num);
                } else if num < maximum {
                    stack.pop();
                    result += 1;
                    continue;
                }
            } else if num != 0 {
                stack.push(num);
            }

            index += 1;
        }

        result + stack.len() as i32
    }
}
