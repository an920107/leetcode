pub struct Solution;

impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        for c in complexity.iter().skip(1) {
            if *c <= complexity[0] {
                return 0;
            }
        }

        let mut result = 1;
        let mut n = complexity.len() - 1;
        while n > 0 {
            result = (result * n) % 1_000_000_007;
            n -= 1;
        }

        result as i32
    }
}
