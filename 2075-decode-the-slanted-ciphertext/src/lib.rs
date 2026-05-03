pub struct Solution;

impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        if encoded_text.len() == 0 {
            return encoded_text;
        }

        let rows = rows as usize;
        let cols = encoded_text.len() / rows;

        let mut grid: Vec<Vec<u8>> = vec![vec![]; rows];
        let encoded_text_bytes = encoded_text.as_bytes();
        for i in 0..rows {
            let char_index = i * cols;
            grid[i] = encoded_text_bytes[char_index..(char_index + cols)].to_vec();
        }

        let mut result = String::new();
        for j in 0..cols {
            for i in 0..rows {
                let pos = (i, j + i);
                if pos.1 >= cols {
                    break;
                }

                result.push(grid[pos.0][pos.1] as char);
            }
        }

        result.trim_end().to_string()
    }
}
