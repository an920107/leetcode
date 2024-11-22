use std::cmp::{max, min};

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left_index = 0;
        let mut right_index = height.len() - 1;

        let mut result = 0;

        while left_index < right_index {
            let left = height[left_index];
            let right = height[right_index];

            result = max(result, min(left, right) * (right_index - left_index) as i32);

            if left < right {
                left_index += 1;
            } else {
                right_index -= 1;
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
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let expected = 49;
        assert_eq!(Solution::max_area(height), expected);
    }

    #[test]
    fn testcase_2() {
        let height = vec![1, 1];
        let expected = 1;
        assert_eq!(Solution::max_area(height), expected);
    }
}

fn main() {}
