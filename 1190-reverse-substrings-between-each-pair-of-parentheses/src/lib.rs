pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack: Vec<u8> = Vec::with_capacity(s.len());
        let mut queue: VecDeque<u8> = VecDeque::with_capacity(s.len());

        for c in s.bytes() {
            if c != b')' {
                stack.push(c);
            } else {
                while let Some(top) = stack.pop()
                    && top != b'('
                {
                    queue.push_back(top);
                }

                while let Some(front) = queue.pop_front() {
                    stack.push(front);
                }
            }
        }

        stack.iter().map(|c| *c as char).collect()
    }
}
