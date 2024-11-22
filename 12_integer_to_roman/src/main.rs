#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn int_to_roman(num: i32) -> String {
        let mut result = String::new();

        let mut num = num;
        while num >= 1000 {
            num -= 1000;
            result.push('M');
        }

        if num >= 900 {
            num -= 900;
            result.push('C');
            result.push('M');
        }

        if num >= 500 {
            num -= 500;
            result.push('D');
        }

        if num >= 400 {
            num -= 400;
            result.push('C');
            result.push('D');
        }

        while num >= 100 {
            num -= 100;
            result.push('C');
        }

        if num >= 90 {
            num -= 90;
            result.push('X');
            result.push('C');
        }

        if num >= 50 {
            num -= 50;
            result.push('L');
        }

        if num >= 40 {
            num -= 40;
            result.push('X');
            result.push('L');
        }

        while num >= 10 {
            num -= 10;
            result.push('X');
        }

        if num >= 9 {
            num -= 9;
            result.push('I');
            result.push('X');
        }

        if num >= 5 {
            num -= 5;
            result.push('V');
        }

        if num >= 4 {
            num -= 4;
            result.push('I');
            result.push('V');
        }

        while num >= 1 {
            num -= 1;
            result.push('I');
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let num = 3749;
        let expected = "MMMDCCXLIX".to_string();
        assert_eq!(Solution::int_to_roman(num), expected);
    }

    #[test]
    fn testcase_2() {
        let num = 58;
        let expected = "LVIII".to_string();
        assert_eq!(Solution::int_to_roman(num), expected);
    }

    #[test]
    fn testcase_3() {
        let num = 1994;
        let expected = "MCMXCIV".to_string();
        assert_eq!(Solution::int_to_roman(num), expected);
    }
}
fn main() {}
