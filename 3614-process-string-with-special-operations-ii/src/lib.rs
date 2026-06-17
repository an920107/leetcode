pub struct Solution;

impl Solution {
    pub fn process_str(s: String, mut k: i64) -> char {
        let mut len = 0;

        for c in s.chars() {
            match c {
                '*' => {
                    if len > 0 {
                        len -= 1;
                    }
                }
                '#' => len *= 2,
                '%' => {}
                _ => len += 1,
            }
        }

        if k >= len {
            return '.';
        }

        for c in s.chars().rev() {
            match c {
                '*' => len += 1,
                '#' => {
                    len /= 2;
                    if len != 0 {
                        k %= len;
                    }
                }
                '%' => k = len - k - 1,
                _ => {
                    if k == len - 1 {
                        return c;
                    } else {
                        len -= 1;
                    }
                }
            }
        }

        '.'
    }
}
