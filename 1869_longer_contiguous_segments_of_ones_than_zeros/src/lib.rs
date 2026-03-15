pub struct Solution;

impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let bytes = s.as_bytes();

        let mut last_b = bytes[0];

        let mut current_lens = if last_b == b'1' { [0, 1] } else { [1, 0] };
        let mut max_lens = current_lens.clone();

        for current_b in s.bytes().skip(1) {
            if current_b == last_b {
                current_lens[(current_b - b'0') as usize] += 1;
            } else {
                current_lens[(current_b - b'0') as usize] = 1;
                current_lens[1 - (current_b - b'0') as usize] = 0;
            }
            max_lens = [
                max_lens[0].max(current_lens[0]),
                max_lens[1].max(current_lens[1]),
            ];

            last_b = current_b;
        }

        max_lens[1] > max_lens[0]
    }
}
