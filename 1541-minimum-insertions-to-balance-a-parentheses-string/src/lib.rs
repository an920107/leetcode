pub struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut result = 0;
        let mut index = 0;
        let mut depth = 0;

        while index < s.len() {
            let c = s.as_bytes()[index];
            match c {
                b'(' => {
                    depth += 1;
                    index += 1;
                }
                b')' => {
                    if depth > 0 {
                        depth -= 1;
                    } else {
                        result += 1;
                    }

                    if index + 1 < s.len() && s.as_bytes()[index + 1] == b')' {
                        index += 2;
                    } else {
                        result += 1;
                        index += 1;
                    }
                }
                _ => unreachable!(),
            }
        }

        result += depth * 2;

        result
    }
}
