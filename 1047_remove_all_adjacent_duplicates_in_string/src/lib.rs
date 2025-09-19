pub struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = String::new();

        for c in s.bytes() {
            if Some(c) == stack.bytes().last() {
                stack.pop();
            } else {
                stack.push(c as char);
            }
        }

        stack
    }
}
