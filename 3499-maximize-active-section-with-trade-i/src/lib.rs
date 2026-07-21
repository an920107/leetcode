pub struct Solution;

impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        let bits: Vec<bool> = [
            vec![true],
            s.bytes().map(|b| b == b'1').collect(),
            vec![true],
        ]
        .concat();
        let n = bits.len();

        let mut left_first_0_indices = vec![None; n];
        let mut left_first_1_indices = vec![None; n];
        let mut right_first_0_indices = vec![None; n];
        let mut right_first_1_indices = vec![None; n];

        let mut last_0_index = None;
        let mut last_1_index = None;
        for (index, &bit) in bits.iter().enumerate() {
            left_first_0_indices[index] = last_0_index;
            left_first_1_indices[index] = last_1_index;
            if bit {
                last_1_index = Some(index);
            } else {
                last_0_index = Some(index);
            }
        }

        let mut last_0_index = None;
        let mut last_1_index = None;
        for (index, &bit) in bits.iter().enumerate().rev() {
            right_first_0_indices[index] = last_0_index;
            right_first_1_indices[index] = last_1_index;
            if bit {
                last_1_index = Some(index);
            } else {
                last_0_index = Some(index);
            }
        }

        let mut best_reversed_0_count = 0;
        for (index, &bit) in bits.iter().enumerate() {
            if bit
                && let Some(left_first_0_index) = left_first_0_indices[index]
                && let Some(right_first_0_index) = right_first_0_indices[index]
                && let Some(left_first_1_index) = left_first_1_indices[left_first_0_index]
                && let Some(right_first_1_index) = right_first_1_indices[right_first_0_index]
            {
                let reversed_0_count = (right_first_1_index - left_first_1_index - 1)
                    - (right_first_0_index - left_first_0_index - 1);
                best_reversed_0_count = best_reversed_0_count.max(reversed_0_count);
            }
        }

        let original_1_count = bits.iter().filter(|b| **b).count() - 2;
        (original_1_count + best_reversed_0_count) as i32
    }
}
