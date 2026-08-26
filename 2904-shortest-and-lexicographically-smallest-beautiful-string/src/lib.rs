pub struct Solution;

const MAX_LEN: usize = 1000;

impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let indices_of_one: Vec<usize> = s
            .bytes()
            .enumerate()
            .filter(|(_, c)| *c == b'1')
            .map(|(index, _)| index)
            .collect();

        let mut result_len = MAX_LEN;
        let mut result_ones: Vec<usize> = vec![];

        for window in indices_of_one.windows(k as usize) {
            let substring_len = window[window.len() - 1] - window[0];
            if substring_len <= result_len {
                let shifted_window: Vec<usize> =
                    window.iter().map(|&num| num - window[0]).collect();
                if substring_len < result_len || shifted_window > result_ones {
                    result_len = substring_len;
                    result_ones = shifted_window.to_vec();
                }
            }
        }

        if result_len == MAX_LEN {
            return String::new();
        }

        let mut result = String::with_capacity(result_len);
        let mut ones_index = 0;
        for num in result_ones[0]..=result_ones[result_ones.len() - 1] {
            if result_ones[ones_index] == num {
                ones_index += 1;
                result.push('1');
            } else {
                result.push('0');
            }
        }
        result
    }
}
