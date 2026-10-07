pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut depth = 0;

        let left_removing_count;
        let mut right_removing_count = 0;

        for c in s.chars() {
            match c {
                '(' => {
                    depth += 1;
                }
                ')' => {
                    if depth > 0 {
                        depth -= 1;
                    } else {
                        right_removing_count += 1;
                    }
                }
                'a'..='z' => {}
                _ => unreachable!(),
            }
        }
        left_removing_count = depth;

        let mut result = HashSet::new();
        Self::recursion(
            &mut result,
            &s.chars().collect::<Vec<char>>(),
            0,
            right_removing_count,
            left_removing_count,
            0,
        );
        result.into_iter().collect()
    }

    fn recursion(
        result: &mut HashSet<String>,
        s: &[char],
        index: usize,
        right_removing_count: usize,
        left_removing_count: usize,
        depth: usize,
    ) {
        if index >= s.len() {
            if depth == 0 {
                result.insert(s.iter().collect());
            }
            return;
        }

        let c = s[index];
        match c {
            '(' => {
                Self::recursion(
                    result,
                    s,
                    index + 1,
                    right_removing_count,
                    left_removing_count,
                    depth + 1,
                );

                if left_removing_count > 0 {
                    let mut new_s = s.to_vec();
                    new_s.remove(index);
                    Self::recursion(
                        result,
                        &new_s,
                        index,
                        right_removing_count,
                        left_removing_count - 1,
                        depth,
                    );
                }
            }
            ')' => {
                if depth > 0 {
                    Self::recursion(
                        result,
                        s,
                        index + 1,
                        right_removing_count,
                        left_removing_count,
                        depth - 1,
                    );
                }

                if right_removing_count > 0 {
                    let mut new_s = s.to_vec();
                    new_s.remove(index);
                    Self::recursion(
                        result,
                        &new_s,
                        index,
                        right_removing_count - 1,
                        left_removing_count,
                        depth,
                    );
                }
            }
            'a'..='z' => {
                Self::recursion(
                    result,
                    s,
                    index + 1,
                    right_removing_count,
                    left_removing_count,
                    depth,
                );
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::remove_invalid_parentheses(")(".to_string()),
        vec![""]
    );
}
