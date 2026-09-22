pub struct Solution;

impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        s.bytes()
            .enumerate()
            .map(|(i, c)| (26 - (c - b'a')) as i32 * (i + 1) as i32)
            .sum::<i32>()
    }
}
