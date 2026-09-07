pub struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let mut memo = vec![vec![None; t.len() + 1]; s.len() + 1];
        Self::recursion(&mut memo, s.as_bytes(), t.as_bytes(), 0, 0)
    }

    fn recursion(
        memo: &mut Vec<Vec<Option<i32>>>,
        s: &[u8],
        t: &[u8],
        s_i: usize,
        t_i: usize,
    ) -> i32 {
        if t_i == t.len() {
            return 1;
        }

        if s_i == s.len() {
            return 0;
        }

        if let Some(result) = memo[s_i][t_i] {
            return result;
        }

        let result = if s[s_i] == t[t_i] {
            Self::recursion(memo, s, t, s_i + 1, t_i)
                + Self::recursion(memo, s, t, s_i + 1, t_i + 1)
        } else {
            Self::recursion(memo, s, t, s_i + 1, t_i)
        };
        memo[s_i][t_i] = Some(result);
        result
    }
}
