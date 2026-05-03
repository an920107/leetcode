pub struct Solution;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".to_string();
        }

        let mut result = String::new();

        if numerator.is_negative() != denominator.is_negative() {
            result.push('-');
        }
        let mut numerator = numerator.unsigned_abs() as u64;
        let mut denominator = denominator.unsigned_abs() as u64;

        let gcd = Solution::gcd(numerator, denominator);
        numerator /= gcd;
        denominator /= gcd;

        result.push_str(&(numerator / denominator).to_string());
        if numerator % denominator == 0 {
            return result;
        }

        result.push('.');

        let mut remains = std::collections::HashMap::<u64, usize>::new();
        loop {
            remains.insert(numerator % denominator, result.len());
            numerator %= denominator;
            numerator *= 10;
            result.push(((numerator / denominator) as u8 + '0' as u8) as char);

            if result.len() >= 10000 {
                break;
            }

            let remain = numerator % denominator;
            if remain == 0 {
                break;
            } else if let Some(index) = remains.get(&remain) {
                result.insert(*index, '(');
                result.push(')');
                break;
            }
        }

        result
    }

    fn gcd(a: u64, b: u64) -> u64 {
        if a == 0 {
            b
        } else if a <= b {
            Solution::gcd(b % a, a)
        } else {
            Solution::gcd(b, a)
        }
    }
}
