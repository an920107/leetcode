pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut character_count: HashMap<u8, i32> = HashMap::new();

        for c in text.bytes() {
            if c == b'b' || c == b'a' || c == b'l' || c == b'o' || c == b'n' {
                *character_count.entry(c).or_default() += 1;
            }
        }

        let b_count = character_count.get(&b'b').copied().unwrap_or_default();
        let a_count = character_count.get(&b'a').copied().unwrap_or_default();
        let l_count = character_count.get(&b'l').copied().unwrap_or_default();
        let o_count = character_count.get(&b'o').copied().unwrap_or_default();
        let n_count = character_count.get(&b'n').copied().unwrap_or_default();

        b_count
            .min(a_count)
            .min(l_count / 2)
            .min(o_count / 2)
            .min(n_count)
    }
}
