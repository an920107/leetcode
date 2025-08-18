pub struct Solution;

impl Solution {
    pub fn is_power_of_four(mut n: i32) -> bool {
        if n <= 0 {
            return false;
        }

        while n > 0 {
            if n == 1 {
                return true
            }
            if n % 4 != 0 {
                return false;
            }
            n /= 4;
        }

        false
    }
}
