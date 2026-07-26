pub struct Solution;

impl Solution {
    pub fn max_product(mut n: i32) -> i32 {
        let mut digits: Vec<i32> = Vec::with_capacity(10);
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.sort_unstable();
        digits.reverse();
        digits[0] * digits[1]
    }
}
