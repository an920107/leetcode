pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut results = [true; 26];
        let mut chars_set: HashSet<u8> = HashSet::new();
        for c in word.bytes() {
            let is_lowercase = c >= b'a' && c <= b'z';
            if is_lowercase && chars_set.contains(&(c - (b'a' - b'A'))) {
                results[(c - b'a') as usize] = false;
            }

            chars_set.insert(c);
        }

        (b'a'..=b'z')
            .zip(b'A'..=b'Z')
            .filter(|(lower, _)| results[(*lower - b'a') as usize])
            .filter(|(lower, upper)| chars_set.contains(lower) && chars_set.contains(upper))
            .count() as i32
    }
}
