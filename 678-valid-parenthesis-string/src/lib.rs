pub struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut star_stack: Vec<usize> = Vec::with_capacity(s.len());
        let mut left_stack: Vec<usize> = Vec::with_capacity(s.len());

        for (index, c) in s.bytes().enumerate() {
            match c {
                b'(' => {
                    left_stack.push(index);
                }
                b')' => {
                    if !left_stack.is_empty() {
                        left_stack.pop();
                    } else if !star_stack.is_empty() {
                        star_stack.pop();
                    } else {
                        return false;
                    }
                }
                b'*' => {
                    star_stack.push(index);
                }
                _ => unreachable!(),
            }
        }

        while let Some(top_left) = left_stack.pop() {
            if let Some(top_star) = star_stack.pop()
                && top_star > top_left
            {
                // nice
            } else {
                return false;
            }
        }

        true
    }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::check_valid_string("()*(()".to_string()), false);
}
