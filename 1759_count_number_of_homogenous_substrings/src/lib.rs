pub struct Solution;

impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let s = s + ".";
        let mut result = 0i64;

        let mut last_ch = s.as_bytes()[0];
        let mut current_len = 1;

        for ch in s.bytes().skip(1) {
            if ch != last_ch {
                result += current_len * (current_len + 1) / 2;
                result %= 1_000_000_007;
                current_len = 1;
            } else {
                current_len += 1;
            }

            last_ch = ch;
        }

        result as i32
    }
}
