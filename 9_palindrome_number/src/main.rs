#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let x_str = x.to_string();
        let s = x_str.as_bytes().to_owned();
        let mut r = x_str.as_bytes().to_owned();
        r.reverse();
        s == r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let x = 121;
        let expected = true;
        assert_eq!(Solution::is_palindrome(x), expected);
    }

    #[test]
    fn testcase_2() {
        let x = -121;
        let expected = false;
        assert_eq!(Solution::is_palindrome(x), expected);
    }

    #[test]
    fn testcase_3() {
        let x = 10;
        let expected = false;
        assert_eq!(Solution::is_palindrome(x), expected);
    }
}

fn main() {}
