pub struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let partition_point = letters.partition_point(|&c| c <= target);
        letters[if partition_point == letters.len() {
            0
        } else {
            partition_point
        }]
    }
}
