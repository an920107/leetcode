pub struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let first_substring = s.get(0..k as usize);
        if first_substring.is_none() {
            return false;
        }

        let mut current_binary = 0u32;
        for c in first_substring.unwrap().bytes() {
            current_binary <<= 1;
            current_binary |= (c == b'1') as u32;
        }

        let mask = (1 << k as u32) - 1;

        let mut seen_binaries: Vec<bool> = vec![false; 1 << k as u32];
        seen_binaries[current_binary as usize] = true;

        for c in s.bytes().skip(k as usize) {
            current_binary <<= 1;
            current_binary &= mask;
            current_binary |= (c == b'1') as u32;
            seen_binaries[current_binary as usize] = true;
        }

        seen_binaries.iter().all(|&b| b)
    }
}
