pub struct Solution;

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let words: Vec<u32> = words
            .iter()
            .map(|word| Solution::convert_to_bits(word))
            .collect();

        let mut result = 0;
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if words[i] == words[j] {
                    result += 1;
                }
            }
        }

        result
    }

    fn convert_to_bits(word: &str) -> u32 {
        let mut result = 0u32;
        for c in word.chars() {
            result |= 1 << c as u32 - 97;
        }
        result
    }
}
