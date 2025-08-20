pub struct Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut allowed_table = 0u32;
        for c in allowed.chars() {
            allowed_table |= 1 << c as u32 - 97;
        }

        let mut result = 0;
        for word in words {
            let mut is_subset = 1;
            for c in word.chars() {
                if allowed_table & 1 << c as u32 - 97 == 0 {
                    is_subset = 0;
                    break;
                }
            }
            result += is_subset;
        }

        result
    }
}
