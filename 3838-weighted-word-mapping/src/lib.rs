pub struct Solution;

impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        let mut result = String::with_capacity(words.len());

        for word in words.into_iter() {
            let weight = word
                .bytes()
                .map(|c| weights[(c - b'a') as usize])
                .sum::<i32>();
            result.push(((25 - weight % 26) as u8 + b'a') as char)
        }

        result
    }
}
