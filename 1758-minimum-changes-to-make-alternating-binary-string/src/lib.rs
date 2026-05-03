pub struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        Solution::start_from(false, &s).min(Solution::start_from(true, &s))
    }

    fn start_from(start: bool, s: &str) -> i32 {
        let mut result = 0;
        let mut required_bit = start;
        for c in s.bytes() {
            if (if required_bit { b'1' } else { b'0' }) != c {
                result += 1;
            }
            required_bit = !required_bit;
        }
        result
    }
}
