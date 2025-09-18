pub struct Solution;

impl Solution {
    pub fn minimized_string_length(s: String) -> i32 {
        let mut result = 0;
        let mut table = vec![false; 26];

        for c in s.chars() {
            let index = c as usize - 'a' as usize;
            if !table[index] {
                result += 1;
                table[index] = true;
            }
        }

        result
    }
}
