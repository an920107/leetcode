pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let knowledge: HashMap<String, String> =
            HashMap::from_iter(knowledge.into_iter().map(|v| (v[0].clone(), v[1].clone())));

        let mut result = String::with_capacity(s.len());
        let mut bracket_buffer = String::with_capacity(s.len());
        let mut bracket_flag = false;
        for c in s.chars() {
            if bracket_flag {
                if c == ')' {
                    bracket_flag = false;
                    if let Some(v) = knowledge.get(&bracket_buffer) {
                        result.push_str(v);
                    } else {
                        result.push('?');
                    }
                    bracket_buffer.clear();
                } else {
                    bracket_buffer.push(c);
                }
            } else {
                if c == '(' {
                    bracket_flag = true;
                } else {
                    result.push(c);
                }
            }
        }

        result
    }
}
