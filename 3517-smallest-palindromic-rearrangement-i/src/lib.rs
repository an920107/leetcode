pub struct Solution;

impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let mut char_count = [0; 26];
        for c in s.bytes() {
            char_count[(c - b'a') as usize] += 1;
        }

        let mut result = String::new();
        let mut mid_char: Option<u8> = None;
        for (index, &count) in char_count.iter().enumerate() {
            let c = index as u8 + b'a';
            if count % 2 == 1 {
                mid_char = Some(c)
            }
            result.push_str(&(c as char).to_string().repeat(count / 2));
        }

        let reversed = result.chars().rev().collect::<String>();
        if let Some(c) = mid_char {
            result.push(c as char);
        }
        result.push_str(&reversed);
        result
    }
}
