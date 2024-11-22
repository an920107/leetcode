#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn my_atoi(s: String) -> i32 {
        let mut s = s.trim_start().to_string();
        if s.len() == 0 {
            return 0;
        }

        let first_c = s.as_bytes()[0] as char;
        let mut neg = false;
        if first_c == '-' {
            neg = true;
            s = s[1..].to_string();
        } else if first_c == '+' {
            s = s[1..].to_string();
        }

        let mut result: i64 = 0;
        for c in s.chars() {
            if !c.is_numeric() {
                break;
            }
            result = result * 10 + c.to_digit(10).unwrap() as i64 * if neg { -1 } else { 1 };
            if result > i32::MAX as i64 {
                return i32::MAX;
            } else if result < i32::MIN as i64 {
                return i32::MIN;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let s = "42".to_string();
        let expected = 42;
        assert_eq!(Solution::my_atoi(s), expected);
    }

    #[test]
    fn testcase_2() {
        let s = "   -042".to_string();
        let expected = -42;
        assert_eq!(Solution::my_atoi(s), expected);
    }

    #[test]
    fn testcase_3() {
        let s = "1337c0d3".to_string();
        let expected = 1337;
        assert_eq!(Solution::my_atoi(s), expected);
    }

    #[test]
    fn testcase_4() {
        let s = "0-1".to_string();
        let expected = 0;
        assert_eq!(Solution::my_atoi(s), expected);
    }

    #[test]
    fn testcase_5() {
        let s = "words and 987".to_string();
        let expected = 0;
        assert_eq!(Solution::my_atoi(s), expected);
    }

    #[test]
    fn testcase_6() {
        let s = "-91283472332".to_string();
        let expected = -2147483648;
        assert_eq!(Solution::my_atoi(s), expected);
    }
    
    #[test]
    fn testcase_7() {
        let s = "".to_string();
        let expected = 0;
        assert_eq!(Solution::my_atoi(s), expected);
    }
}

fn main() {}
