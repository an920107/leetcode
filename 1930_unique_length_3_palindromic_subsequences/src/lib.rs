pub struct Solution;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let bytes = s.as_bytes();
        let a_byte = 'a' as u8;
        let z_byte = 'z' as u8;

        // aba -> a: row, b: col
        let mut table: Vec<u32> = vec![0; 26];
        let mut result = 0;

        for first_c in a_byte..=z_byte {
            let mut has_first_c_found = false;
            let mut first_index = 0;
            for (index, c) in s.bytes().enumerate() {
                if first_c == c {
                    first_index = index;
                    has_first_c_found = true;
                    break;
                }
            }
            if !has_first_c_found {
                continue;
            }

            let mut has_last_c_found = false;
            let mut last_index = s.len() - 1;
            while last_index > first_index {
                if bytes[last_index] == first_c {
                    has_last_c_found = true;
                    break;
                }
                last_index -= 1;
            }
            if !has_last_c_found {
                continue;
            }

            for &c in s.as_bytes()[first_index + 1..last_index].iter() {
                let has_taken = table[(first_c - a_byte) as usize] & 1 << c > 0;
                if has_taken {
                    continue;
                }

                result += 1;
                table[(first_c - a_byte) as usize] |= 1 << c;
            }
        }

        result
    }
}
