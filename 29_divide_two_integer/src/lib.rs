pub struct Solution;

impl Solution {
    /// - Dividend = Quotient * Divisor + Rem
    /// - Rem < Divisor
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        if divisor == i32::MIN {
            return if dividend == i32::MIN { 1 } else { 0 };
        }

        let is_positive = dividend.is_positive() == divisor.is_positive();

        let mut is_overflow = false;
        if dividend == i32::MIN {
            is_overflow = true;
            dividend += 1;
        }

        dividend = dividend.abs();
        divisor = divisor.abs();

        if dividend < divisor {
            return 0;
        }

        let mut quotient = 0;
        let mut rem;
        loop {
            let mut exp = 1;
            while divisor << exp <= dividend && divisor << exp > 0 {
                exp += 1;
            }
            exp -= 1;

            quotient += 1 << exp;
            rem = dividend - (divisor << exp);
            if rem < divisor {
                break;
            }

            dividend = rem;
        }

        if is_overflow && rem + 1 >= divisor {
            if is_positive {
                if quotient == i32::MAX {
                    quotient
                } else {
                    quotient + 1
                }
            } else {
                -quotient - 1
            }
        } else {
            if is_positive { quotient } else { -quotient }
        }
    }
}
