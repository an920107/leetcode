pub struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = Vec::new();
        let parts = path.split('/');

        for part in parts {
            match part {
                "" | "." => {}
                ".." => {
                    stack.pop();
                }
                _ => {
                    stack.push(part);
                }
            }
        }

        let mut result = String::new();
        for part in stack {
            result.push('/');
            result.push_str(part);
        }

        if result.len() == 0 {
            result.push('/');
        }

        result
    }
}
