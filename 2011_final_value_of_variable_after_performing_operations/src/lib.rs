pub struct Solution;

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut result = 0;

        for op in operations {
            if op.contains('+') {
                result += 1;
            } else {
                result -= 1;
            }
        }

        result
    }
}
