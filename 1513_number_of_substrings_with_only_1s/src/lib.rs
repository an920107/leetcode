pub struct Solution;

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let s = s + "0";
        let mut result = 0i64;

        let mut last_value = s.as_bytes()[0] as char;
        let mut current_len = if last_value == '1' { 1 } else { 0 };
        for value in s.bytes().skip(1) {
            let value = value as char;
            if value == '0' {
                if last_value == '1' {
                    result += current_len * (current_len + 1) / 2;
                    result %= 1_000_000_007;
                    current_len = 0;
                }
            } else {
                current_len += 1;
            }
            last_value = value;
        }

        result as i32
    }
}
