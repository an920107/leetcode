use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let jewels: HashSet<char> = HashSet::from_iter(jewels.chars());
        let mut result = 0;

        for stone in stones.chars() {
            if jewels.contains(&stone) {
                result += 1;
            }
        }

        result
    }
}
