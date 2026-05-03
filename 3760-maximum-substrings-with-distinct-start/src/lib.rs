pub struct Solution;

impl Solution {
    pub fn max_distinct(s: String) -> i32 {
        let mut result = 0;
        let mut seen_chars = 0;
        for c in s.bytes() {
            let mask = 1 << (c - b'0') as i32;
            if seen_chars & mask == 0 {
                seen_chars |= mask;
                result += 1;
            }
        }
        result
    }
}
