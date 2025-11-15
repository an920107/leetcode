pub struct Solution;

impl Solution {
    // O(N * sqrt(N))
    pub fn number_of_substrings(s: String) -> i32 {
        let utils = Utils::new(&s);

        let next_zero_indices = utils.next_zero_indices();

        let mut result = 0;

        // O(N)
        for (l_index, _) in utils.bits.iter().enumerate() {
            let mut r_index = l_index;
            let mut zero_count = if utils.bits[l_index] { 0 } else { 1 };

            // O(sqrt(N))
            while r_index < utils.bits.len() && zero_count * zero_count <= utils.bits.len() {
                let next_zero_index = next_zero_indices[r_index];

                let current_size = next_zero_index - l_index;
                let window_size = next_zero_index - r_index;

                let one_count = current_size - zero_count;

                if one_count >= zero_count * zero_count {
                    result += window_size.min(one_count - zero_count * zero_count + 1);
                }

                r_index = next_zero_index;
                zero_count += 1;
            }
        }

        result as i32
    }
}

struct Utils {
    bits: Vec<bool>,
}

impl Utils {
    pub fn new(s: &str) -> Utils {
        Self {
            bits: s.bytes().map(|c| c > '0' as u8).collect(),
        }
    }

    pub fn next_zero_indices(&self) -> Vec<usize> {
        let mut indices_of_next_zero: Vec<usize> = vec![self.bits.len(); self.bits.len()];

        // from the last to the first
        for (index, _) in self.bits.iter().enumerate().rev().skip(1) {
            indices_of_next_zero[index] = if self.bits[index + 1] {
                // the next bit is one, using the index of the next bit
                indices_of_next_zero[index + 1]
            } else {
                // the next bit is zero
                index + 1
            }
        }

        indices_of_next_zero
    }
}
