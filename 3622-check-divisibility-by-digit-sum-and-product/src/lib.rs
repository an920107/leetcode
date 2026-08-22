pub struct Solution;

impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let mut digits_sum = 0;
        let mut digits_product = 1;

        {
            let mut n = n;
            while n > 0 {
                let digit = n % 10;
                digits_sum += digit;
                digits_product *= digit;
                n /= 10;
            }
        }

        n % (digits_sum + digits_product) == 0
    }
}
