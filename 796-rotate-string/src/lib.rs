pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let mut s = VecDeque::from_iter(s.bytes());
        let goal = VecDeque::from_iter(goal.bytes());

        let n = s.len();
        for _ in 0..n {
            if s == goal {
                return true;
            }
            let c = s.pop_front().unwrap();
            s.push_back(c);
        }

        false
    }
}
