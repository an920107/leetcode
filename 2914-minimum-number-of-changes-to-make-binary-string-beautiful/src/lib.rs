pub struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut result = 0;

        for (i, window) in s.as_bytes().windows(2).enumerate() {
            let last_c = window[0];
            let current_c = window[1];

            if last_c != current_c && i % 2 == 0 {
                result += 1;
            }
        }

        result
    }
}
