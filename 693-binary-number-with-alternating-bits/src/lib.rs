pub struct Solution;

impl Solution {
    pub fn has_alternating_bits(mut n: i32) -> bool {
        let mut last_bit = n & 1;
        while n > 0 {
            n >>= 1;
            if n & 1 == last_bit {
                return false;
            }
            last_bit = n & 1;
        }
        true
    }
}
