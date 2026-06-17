pub struct Solution;

impl Solution {
    pub fn process_str(s: String) -> String {
        let mut result = String::new();

        for c in s.chars() {
            match c {
                '*' => {
                    result.pop();
                }
                '#' => result.push_str(&result.clone()),
                '%' => result = result.chars().rev().collect(),
                _ => result.push(c),
            }
        }

        result
    }
}
