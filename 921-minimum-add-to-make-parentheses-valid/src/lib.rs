pub struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut result = 0;
        let mut depth = 0;

        for c in s.bytes() {
            if c == b'(' {
                depth += 1;
            } else {
                if depth > 0 {
                    depth -= 1;
                } else {
                    result += 1;
                }
            }
        }

        result + depth
    }
}
