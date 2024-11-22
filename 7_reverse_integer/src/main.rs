#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reverse(x: i32) -> i32 {
        let s = if x < 0 {
            x.to_string()[1..].to_owned() + "-"
        } else {
            x.to_string().to_owned()
        };

        match s.chars().rev().collect::<String>().parse::<i32>() {
            Ok(result) => result,
            Err(_) => 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let x = 123;
        let expected = 321;
        assert_eq!(Solution::reverse(x), expected);
    }

    #[test]
    fn testcase_2() {
        let x = -123;
        let expected = -321;
        assert_eq!(Solution::reverse(x), expected);
    }
    #[test]
    fn testcase_3() {
        let x = 120;
        let expected = 21;
        assert_eq!(Solution::reverse(x), expected);
    }
}

fn main() {}
