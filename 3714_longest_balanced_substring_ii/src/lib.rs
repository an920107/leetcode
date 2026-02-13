pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let one = Solution::one(&s);
        let two = Solution::two(&s, b'a')
            .max(Solution::two(&s, b'b'))
            .max(Solution::two(&s, b'c'));
        let three = Solution::three(&s);

        one.max(two).max(three)
    }

    fn one(s: &str) -> i32 {
        let mut last_c = s.as_bytes()[0];
        let mut result = 1;
        let mut current_len = 1;

        for c in s.bytes().skip(1) {
            if last_c == c {
                current_len += 1;
                result = result.max(current_len);
            } else {
                last_c = c;
                current_len = 1;
            }
        }

        result
    }

    fn two(s: &str, except_c: u8) -> i32 {
        let mut counts: Vec<Vec<i32>> = vec![vec![0; 3]];
        let mut relations: HashMap<i32, usize> = HashMap::from([(0, 0)]);

        let mut result = 0;

        for (index, c) in s.bytes().enumerate() {
            if c == except_c {
                counts = vec![vec![0; 3]];
                relations = HashMap::from([(0, index + 1)]);
                continue;
            }

            counts.push(counts.last().unwrap().clone());
            counts.last_mut().unwrap()[(c - b'a') as usize] += 1;

            let last_count = counts.last().unwrap();
            let relation = match except_c {
                b'a' => last_count[1] - last_count[2],
                b'b' => last_count[0] - last_count[2],
                _ => last_count[0] - last_count[1],
            };
            if !relations.contains_key(&relation) {
                relations.insert(relation, index + 1);
            }

            let current_len = (index + 1) - relations.get(&relation).unwrap_or(&(index + 1));
            result = result.max(current_len);
        }

        result as i32
    }

    fn three(s: &str) -> i32 {
        let mut has_seen = 0; // 7 if all of them has been seen
        let mut counts: Vec<Vec<i32>> = vec![vec![0; 3]];
        let mut relations: HashMap<(i32, i32), usize> = HashMap::from([((0, 0), 0)]);

        let mut result = 0;

        for (index, c) in s.bytes().enumerate() {
            has_seen |= match c {
                b'a' => 1,
                b'b' => 2,
                _ => 4,
            };

            counts.push(counts.last().unwrap().clone());
            counts.last_mut().unwrap()[(c - b'a') as usize] += 1;

            let last_count = counts.last().unwrap();
            let relation = (last_count[0] - last_count[1], last_count[0] - last_count[2]);
            if !relations.contains_key(&relation) {
                relations.insert(relation, index + 1);
            }

            if has_seen != 7 {
                continue;
            }

            let current_len = (index + 1) - relations.get(&relation).unwrap_or(&(index + 1));
            result = result.max(current_len);
        }

        result as i32
    }
}
