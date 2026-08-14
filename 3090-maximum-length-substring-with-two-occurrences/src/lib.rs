pub struct Solution;

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let mut result = 1;
        let mut left_index = 0;
        let mut right_index = 0;
        let mut frequency = [0; 26];
        let bytes: Vec<u8> = s.bytes().collect();
        frequency[(bytes[0] - b'a') as usize] = 1;
        while right_index < bytes.len() - 1 {
            right_index += 1;
            let current_char = bytes[right_index] - b'a';
            frequency[current_char as usize] += 1;
            while frequency[current_char as usize] > 2 {
                let removed_char = bytes[left_index] - b'a';
                frequency[removed_char as usize] -= 1;
                left_index += 1;
            }
            result = result.max(right_index - left_index + 1);
        }
        result as i32
    }
}
