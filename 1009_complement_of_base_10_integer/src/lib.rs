pub struct Solution;

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let offset = n.leading_zeros();
        !n << offset >> offset
    }
}
