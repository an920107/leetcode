pub struct Solution;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let mut result = 0i64;

        for k in 1..=n {
            result <<= Solution::dig(k);
            result |= k as i64;
            result %= 1_000_000_007;
        }

        result as i32
    }

    #[inline]
    fn dig(n: i32) -> i32 {
        32 - n.leading_zeros() as i32
    }
}
