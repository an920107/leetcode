pub struct Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        s.find('0')
            .and_then(|i| Some(s[i..s.len()].bytes().all(|b| b == b'0')))
            .unwrap_or(true)
    }
}
