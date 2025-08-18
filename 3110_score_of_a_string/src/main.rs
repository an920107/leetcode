fn main() {
    println!("{}", Solution::score_of_string("hello".to_string()));
}

struct Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        s.chars()
            .zip(s.chars().skip(1))
            .map(|(a, b)| (a as i32 - b as i32).abs())
            .sum()
    }
}
