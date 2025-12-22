pub struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let n = strs[0].len();

        let mut index = 0;
        let mut result = 0;
        while index < n {
            let mut last_char = 0u8;
            for str in strs.iter() {
                let current_char = str.as_bytes()[index];
                if current_char < last_char {
                    result += 1;
                    break;
                }
                last_char = current_char;
            }
            index += 1;
        }

        result
    }
}
