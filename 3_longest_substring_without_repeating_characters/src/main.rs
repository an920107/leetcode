#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut deq = std::collections::VecDeque::<char>::new();
        let mut set = std::collections::HashSet::<char>::new();

        let mut max_len = 0;

        for c in s.chars() {
            deq.push_back(c);
            if set.contains(&c) {
                loop {
                    let front = deq.pop_front();
                    if let Some(f) = front {
                        set.remove(&f);
                    }
                    if front == Some(c) {
                        break;
                    }
                }
            }
            set.insert(c);
            max_len = std::cmp::max(max_len, deq.len());
        }

        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let s = "abcabcbb".to_string();
        let expected = 3;
        assert_eq!(Solution::length_of_longest_substring(s), expected);
    }

    #[test]
    fn testcase_2() {
        let s = "bbbbb".to_string();
        let expected = 1;
        assert_eq!(Solution::length_of_longest_substring(s), expected);
    }

    #[test]
    fn testcase_3() {
        let s = "pwwkew".to_string();
        let expected = 3;
        assert_eq!(Solution::length_of_longest_substring(s), expected);
    }
}

fn main() {}
