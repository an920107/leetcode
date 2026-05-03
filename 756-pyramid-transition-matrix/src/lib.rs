/*
 * @lc app=leetcode id=756 lang=rust
 *
 * [756] Pyramid Transition Matrix
 */

pub struct Solution;

// @lc code=start
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let n = bottom.len();

        let mut pyramid: Vec<Vec<u8>> = vec![vec![0; n]; n];
        for (index, c) in bottom.bytes().enumerate() {
            pyramid[n - 1][index] = c;
        }

        let mut components: HashMap<(u8, u8), Vec<u8>> = HashMap::new();
        for raw_component in allowed {
            let bytes = raw_component.as_bytes();
            let key = (bytes[0], bytes[1]);
            let val = bytes[2];
            components.entry(key).or_insert(vec![]).push(val);
        }

        let mut impossible_rows: HashSet<Vec<u8>> = HashSet::new();

        Solution::recursion(pyramid, (n - 2, 0), &mut impossible_rows, &components)
    }

    fn recursion(
        pyramid: Vec<Vec<u8>>,
        pos: (usize, usize),
        impossible_rows: &mut HashSet<Vec<u8>>,
        components: &HashMap<(u8, u8), Vec<u8>>,
    ) -> bool {
        let mut result = false;

        let next_pos = Solution::count_next_pos(pos);

        let base_row = &pyramid[pos.0 + 1];
        let key = (base_row[pos.1], base_row[pos.1 + 1]);

        if pos.1 == 0 && impossible_rows.contains(&pyramid[pos.0 + 1]) {
            return false;
        }

        for &component_top in components.get(&key).or(Some(&vec![])).unwrap() {
            let mut new_pyramid = pyramid.clone();
            new_pyramid[pos.0][pos.1] = component_top;

            if let Some(next_pos) = next_pos {
                result = result
                    || Solution::recursion(new_pyramid, next_pos, impossible_rows, components);
            } else {
                result = true;
            }

            if result {
                break;
            }
        }

        if pos.1 == 0 && !result {
            impossible_rows.insert(pyramid[pos.0 + 1].clone());
        }

        result
    }

    fn count_next_pos(current_pos: (usize, usize)) -> Option<(usize, usize)> {
        if current_pos.0 == current_pos.1 {
            if current_pos.0 == 0 {
                None
            } else {
                Some((current_pos.0 - 1, 0))
            }
        } else {
            Some((current_pos.0, current_pos.1 + 1))
        }
    }
}
// @lc code=end
