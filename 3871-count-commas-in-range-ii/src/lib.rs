pub struct Solution;

impl Solution {
    pub fn count_commas(mut n: i64) -> i64 {
        let mut result = 0;

        if n >= 1_000_000_000_000_000 {
            result += (n - 1_000_000_000_000_000 + 1) * 5;
            n = 999_999_999_999_999;
        }
        if n >= 1_000_000_000_000 {
            result += (n - 1_000_000_000_000 + 1) * 4;
            n = 999_999_999_999;
        }
        if n >= 1_000_000_000 {
            result += (n - 1_000_000_000 + 1) * 3;
            n = 999_999_999;
        }
        if n >= 1_000_000 {
            result += (n - 1_000_000 + 1) * 2;
            n = 999_999;
        }
        if n >= 1_000 {
            result += n - 1_000 + 1;
        }
        result
    }
}
