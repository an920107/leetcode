pub struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let bits: Vec<bool> = s.bytes().map(|c| c == b'1').collect();

        let mut last_bit = bits[0];
        let mut last_len = 0;
        let mut current_len = 1;
        let mut result = 0;

        for &bit in bits.iter().skip(1) {
            if bit == last_bit {
                current_len += 1;
                continue;
            }

            result += last_len.min(current_len);
            last_bit = bit;
            last_len = current_len;
            current_len = 1;
        }

        result + last_len.min(current_len)
    }
}
