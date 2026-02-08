pub struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut stack: Vec<u8> = vec![];

        let mut delete_count = 0;
        for c in s.bytes() {
            if c == b'a'
                && let Some(&top) = stack.last()
                && top == b'b'
            {
                stack.pop();
                delete_count += 1;
            } else {
                stack.push(c);
            }
        }

        delete_count
    }
}
