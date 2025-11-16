pub struct Solution;

impl Solution {
    pub fn max_power(mut s: String) -> i32 {
        s.push('.');
        let mut result = 0;

        let mut last_ch = s.as_bytes()[0];
        let mut current_length = 1;

        for ch in s.bytes().skip(1) {
            if last_ch != ch {
                result = result.max(current_length);
                current_length = 1;
            } else {
                current_length += 1;
            }

            last_ch = ch;
        }

        result
    }
}
