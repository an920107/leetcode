pub struct Solution;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        for length in (1..=s.len()).rev() {
            let mut char_count: Vec<i32> = vec![0; 26];

            for index in 0..length {
                let c = s.as_bytes()[index];
                char_count[(c - b'a') as usize] += 1;
            }
            if Solution::is_balanced(&char_count) {
                return length as i32;
            }

            for index in 1..(s.len() - length + 1) {
                let last_c = s.as_bytes()[index - 1];
                char_count[(last_c - b'a') as usize] -= 1;
                let next_c = s.as_bytes()[index + length - 1];
                char_count[(next_c - b'a') as usize] += 1;

                if Solution::is_balanced(&char_count) {
                    return length as i32;
                }
            }
        }

        0
    }

    fn is_balanced(char_count: &Vec<i32>) -> bool {
        let mut count = 0;
        for c in 0..26 {
            if char_count[c] == 0 {
                continue;
            }

            if count == 0 {
                count = char_count[c];
            } else if count != char_count[c] {
                return false;
            }
        }
        true
    }
}
