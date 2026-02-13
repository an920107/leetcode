pub struct Solution;

use std::collections::HashMap;

// generate 32-bit prime by `openssl prime -generate -bits 32`
const W1: i64 = 4031870903;
const W2: i64 = 3649503757;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let weights: Vec<Vec<i64>> = vec![
            vec![0, W1, W2],        // a
            vec![W1, 0, W2],        // b
            vec![W1, W2, 0],        // c
            vec![W1, -W1, W2],      // ab
            vec![W1, W2, -W1],      // ac
            vec![W2, W1, -W1],      // bc
            vec![W1, W2, -W1 - W2], // abc
        ];

        weights
            .iter()
            .map(|w| Solution::get_max_length_for_weights(&s, &w))
            .max()
            .unwrap_or(0)
    }

    fn get_max_length_for_weights(s: &str, char_weights: &[i64]) -> i32 {
        let mut first_seen: HashMap<i64, usize> = HashMap::from([(0, 0)]);
        let mut current_hash = 0i64;
        let mut result = 0;

        for (index, c) in s.bytes().enumerate() {
            current_hash += char_weights[(c - b'a') as usize];
            if let Some(last_index) = first_seen.get(&current_hash) {
                result = result.max(index + 1 - last_index);
            } else {
                first_seen.insert(current_hash, index + 1);
            }
        }

        result as i32
    }
}
