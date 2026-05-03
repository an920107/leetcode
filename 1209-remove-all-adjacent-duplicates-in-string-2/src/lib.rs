pub struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack = String::new();
        let mut dup_count_stack = Vec::<i32>::new();
        let mut dup_count = 1;

        for c in s.bytes() {
            if stack.is_empty() {
                stack.push(c as char);
            } else if c == stack.bytes().last().unwrap() {
                dup_count += 1;
                if dup_count == k {
                    while dup_count > 1 {
                        stack.pop();
                        dup_count -= 1;
                    }
                    dup_count = dup_count_stack.pop().unwrap_or(1);
                } else {
                    stack.push(c as char);
                }
            } else {
                stack.push(c as char);
                dup_count_stack.push(dup_count);
                dup_count = 1;
            }
        }

        stack
    }
}
