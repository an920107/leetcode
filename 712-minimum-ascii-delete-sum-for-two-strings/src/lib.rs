pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let mut memo = HashMap::new();
        Solution::recursion((0, 0), (&s1, &s2), &mut memo)
    }

    fn recursion(
        i: (usize, usize),
        s: (&str, &str),
        memo: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if let Some(result) = memo.get(&i) {
            return *result;
        }

        if i.0 >= s.0.len() {
            return s.1.as_bytes()[i.1..s.1.len()]
                .iter()
                .map(|&c| c as i32)
                .sum();
        }

        if i.1 >= s.1.len() {
            return s.0.as_bytes()[i.0..s.0.len()]
                .iter()
                .map(|&c| c as i32)
                .sum();
        }

        let c = (
            s.0.as_bytes().get(i.0).copied().unwrap(),
            s.1.as_bytes().get(i.1).copied().unwrap(),
        );

        let result: i32;

        if c.0 == c.1 {
            result = Self::recursion((i.0 + 1, i.1 + 1), s, memo);
        } else {
            result = std::cmp::min(
                c.0 as i32 + Self::recursion((i.0 + 1, i.1), s, memo),
                c.1 as i32 + Self::recursion((i.0, i.1 + 1), s, memo),
            );
        }

        memo.insert(i, result);
        result
    }
}
