pub struct Solution;

impl Solution {
    pub fn sum_game(num: String) -> bool {
        let n = num.len();
        let digits = num.as_bytes();

        let left_mark_count = digits[..(n / 2)].iter().filter(|&&c| c == b'?').count() as i32;
        let right_mark_count = digits[(n / 2)..].iter().filter(|&&c| c == b'?').count() as i32;
        let left_sum: i32 = digits[..(n / 2)]
            .iter()
            .filter(|&&c| c != b'?')
            .map(|&c| (c - b'0') as i32)
            .sum();
        let right_sum: i32 = digits[(n / 2)..]
            .iter()
            .filter(|&&c| c != b'?')
            .map(|&c| (c - b'0') as i32)
            .sum();

        if (left_mark_count + right_mark_count) % 2 == 1 {
            return true;
        }

        let mark_count_diff = left_mark_count - right_mark_count;
        let sum_diff = right_sum - left_sum;

        if mark_count_diff / 2 * 9 != sum_diff {
            return true;
        }

        false
    }
}
