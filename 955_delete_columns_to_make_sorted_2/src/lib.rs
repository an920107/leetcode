pub struct Solution;

impl Solution {
    pub fn min_deletion_size(mut strs: Vec<String>) -> i32 {
        let mut result = 0;

        let mut index = 0;
        let mut last_char = 0u8;

        let mut has_to_check = vec![true; strs.len()];
        has_to_check[0] = false;

        while index < strs.first().unwrap().len() {
            let mut need_to_remove = false;
            let mut tmp_has_to_check = has_to_check.clone();

            for (j, str) in strs.iter().enumerate() {
                let current_char = str.as_bytes()[index];

                if !has_to_check[j] {
                    last_char = current_char;
                    continue;
                }

                if current_char < last_char {
                    need_to_remove = true;
                    break;
                } else if current_char > last_char {
                    tmp_has_to_check[j] = false;
                }

                last_char = current_char;
            }

            if need_to_remove {
                result += 1;
                strs.iter_mut().for_each(|s| {
                    s.remove(index);
                });
            } else {
                has_to_check = tmp_has_to_check;
                index += 1;
            }
        }

        result
    }
}
