pub struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut a_counts = vec![0; s.len() + 1];

        for (index, c) in s.bytes().enumerate() {
            if c == 'a' as u8 {
                a_counts[index + 1] = a_counts[index] + 1;
            } else {
                a_counts[index + 1] = a_counts[index];
            }
        }

        let mut result = i32::MAX;
        for index in 0..(s.len() + 1) {
            let left_b_count = index - a_counts[index];
            let right_a_count = a_counts.last().unwrap() - a_counts[index];
            result = result.min((left_b_count + right_a_count) as i32);
        }

        result
    }
}
