pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<u8> = Vec::with_capacity(s.len());
        let pair_map: HashMap<u8, u8> =
            HashMap::from_iter([(b'(', b')'), (b'[', b']'), (b'{', b'}')]);

        for c in s.bytes() {
            match c {
                b'(' | b'[' | b'{' => {
                    stack.push(c);
                }
                b')' | b']' | b'}' => {
                    if let Some(top) = stack.pop() {
                        if pair_map.get(&top) != Some(&c) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }

        stack.is_empty()
    }
}
