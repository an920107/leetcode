pub struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut x = 0;
        let mut y = 0;

        for c in moves.bytes() {
            match c {
                b'R' => x += 1,
                b'L' => x -= 1,
                b'U' => y += 1,
                b'D' => y -= 1,
                _ => unreachable!(),
            };
        }

        x == 0 && y == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::judge_circle("LRDU".to_string()), true);
        assert_eq!(Solution::judge_circle("L".to_string()), false);
    }
}
