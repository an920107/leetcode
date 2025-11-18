pub struct Solution;

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut result = false;

        let mut index = 0;
        while index < bits.len() {
            let bit = bits[index];

            if bit == 1 {
                index += 2;
                result = false;
            } else {
                index += 1;
                result = true;
            }
        }

        result
    }
}

