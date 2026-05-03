pub struct Solution;

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.chars().max().unwrap() as i32 - 48
    }
}
