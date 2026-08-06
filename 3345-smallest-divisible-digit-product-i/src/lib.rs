pub struct Solution;

impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        (n..)
            .filter(|&m| Self::product_of_digits(m) % t == 0)
            .next()
            .unwrap()
    }

    fn product_of_digits(mut n: i32) -> i32 {
        let mut result = 1;
        while n > 0 {
            result *= n % 10;
            n /= 10
        }
        result
    }
}
