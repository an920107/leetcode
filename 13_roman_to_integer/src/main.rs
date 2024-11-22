#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;

        let mut deq = std::collections::VecDeque::from_iter(s.chars());

        while let Some(c) = deq.pop_front() {
            result += match c {
                'M' => 1000,
                'D' => 500,
                'C' => {
                    let next = deq.front().cloned();
                    if next == Some('M') {
                        deq.pop_front();
                        900
                    } else if next == Some('D') {
                        deq.pop_front();
                        400
                    } else {
                        100
                    }
                }
                'L' => 50,
                'X' => {
                    let next = deq.front().cloned();
                    if next == Some('C') {
                        deq.pop_front();
                        90
                    } else if next == Some('L') {
                        deq.pop_front();
                        40
                    } else {
                        10
                    }
                }
                'V' => 5,
                'I' => {
                    let next = deq.front().cloned();
                    if next == Some('X') {
                        deq.pop_front();
                        9
                    } else if next == Some('V') {
                        deq.pop_front();
                        4
                    } else {
                        1
                    }
                }
                _ => 0,
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
        let s = "III".to_string();
        let expected = 3;
        assert_eq!(Solution::roman_to_int(s), expected);
    }

    #[test]
    fn testcase_2() {
        let s = "LVIII".to_string();
        let expected = 58;
        assert_eq!(Solution::roman_to_int(s), expected);
    }

    #[test]
    fn testcase_3() {
        let s = "MCMXCIV".to_string();
        let expected = 1994;
        assert_eq!(Solution::roman_to_int(s), expected);
    }
}

fn main() {}
