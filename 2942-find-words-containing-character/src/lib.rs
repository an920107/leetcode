pub struct Solution;

impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        words
            .iter()
            .enumerate()
            .filter(|(_, word)| word.contains(x))
            .map(|(index, _)| index as i32)
            .collect()
    }
}
