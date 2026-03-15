pub struct Solution;

impl Solution {
    pub fn reverse_prefix(s: String, k: i32) -> String {
        let mut prefix = s[0..(k as usize)].as_bytes().to_vec();
        prefix.reverse();
        String::from_utf8_lossy(&prefix).to_string() + &s[(k as usize)..s.len()]
    }
}
