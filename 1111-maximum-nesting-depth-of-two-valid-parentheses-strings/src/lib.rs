pub struct Solution;

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut max_depth = 0;
        let mut current_depth = 0;
        for c in seq.bytes() {
            if c == b'(' {
                current_depth += 1;
                max_depth = max_depth.max(current_depth);
            } else if c == b')' {
                current_depth -= 1;
            }
        }

        let target_max_depth = (max_depth + 1) / 2;
        let mut current_depth = 0;
        let mut result: Vec<i32> = Vec::with_capacity(seq.len());
        for c in seq.bytes() {
            if c == b'(' {
                if current_depth < target_max_depth {
                    current_depth += 1;
                    result.push(0);
                } else {
                    result.push(1);
                }
            } else if c == b')' {
                if current_depth > 0 {
                    current_depth -= 1;
                    result.push(0);
                } else {
                    result.push(1);
                }
            }
        }
        result
    }
}
