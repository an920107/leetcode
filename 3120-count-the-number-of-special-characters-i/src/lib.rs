pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let chars_set: HashSet<u8> = HashSet::from_iter(word.bytes());
        (b'a'..=b'z')
            .zip(b'A'..=b'Z')
            .filter(|(lower, upper)| chars_set.contains(lower) && chars_set.contains(upper))
            .count() as i32
    }
}
