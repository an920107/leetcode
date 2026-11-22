pub struct Solution;

impl Solution {
    pub fn check_string(s: String) -> bool {
        let mut b_is_seen = false;

        for c in s.bytes() {
            match c {
                b'a' => {
                    if b_is_seen {
                        return false;
                    }
                }
                b'b' => {
                    b_is_seen = true;
                }
                _ => unreachable!(),
            }
        }

        true
    }
}
