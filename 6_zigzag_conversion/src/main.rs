#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut result = String::new();

        let mut leads = Vec::<i32>::new();
        for (i, c) in s.chars().enumerate() {
            let i = i as i32;
            if i % (num_rows * 2 - 2) == 0 {
                leads.push(i);
                result.push(c);
            }
        }

        for offset in 1..num_rows - 1 {
            for i in leads.to_owned() {
                let j = i + offset;
                if j < s.len() as i32 {
                    result.push(s.as_bytes()[j as usize] as char);
                }
                let j = j + (num_rows - offset) * 2 - 2;
                if j < s.len() as i32 {
                    result.push(s.as_bytes()[j as usize] as char);
                }
            }
        }

        for i in leads.to_owned() {
            let j = i + num_rows - 1;
            if j < s.len() as i32 {
                result.push(s.as_bytes()[j as usize] as char);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let s = "PAYPALISHIRING".to_string();
        let rows = 3;
        let expected = "PAHNAPLSIIGYIR".to_string();
        assert_eq!(Solution::convert(s, rows), expected);
    }

    #[test]
    fn testcase_2() {
        let s = "PAYPALISHIRING".to_string();
        let rows = 4;
        let expected = "PINALSIGYAHRPI".to_string();
        assert_eq!(Solution::convert(s, rows), expected);
    }

    #[test]
    fn testcase_3() {
        let s = "A".to_string();
        let rows = 1;
        let expected = "A".to_string();
        assert_eq!(Solution::convert(s, rows), expected);
    }

    #[test]
    fn testcase_4() {
        let s = "AB".to_string();
        let rows = 2;
        let expected = "AB".to_string();
        assert_eq!(Solution::convert(s, rows), expected);
    }

    #[test]
    fn testcase_5() {
        let s = "ABCD".to_string();
        let rows = 3;
        let expected = "ABDC".to_string();
        assert_eq!(Solution::convert(s, rows), expected);
    }
}

fn main() {}
