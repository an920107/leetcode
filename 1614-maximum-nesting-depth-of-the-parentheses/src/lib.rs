pub struct Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut depth = 0;
        let mut result = 0;

        for c in s.bytes() {
            if c == b'(' {
                depth += 1;
                result = result.max(depth);
            } else if c == b')' {
                depth -= 1;
            }
        }

        result
    }
}
