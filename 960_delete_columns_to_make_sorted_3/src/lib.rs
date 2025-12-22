pub struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let n = strs[0].len();
        let mut memo: Vec<Vec<Option<usize>>> = vec![vec![None; n]; n + 1];
        let ordered_str_len = Solution::recursion(0, 0, &mut memo, &strs);
        (strs[0].len() - ordered_str_len) as i32
    }

    fn recursion(
        index: usize,
        len: usize,
        memo: &mut Vec<Vec<Option<usize>>>,
        strs: &Vec<String>,
    ) -> usize {
        if let Some(result) = memo[len][index] {
            return result;
        }

        let mut available_lens: Vec<usize> = vec![];

        let current_col = if len > 0 {
            strs.iter().map(|s| s.as_bytes()[index]).collect()
        } else {
            vec![0; strs.len()]
        };

        let mut next_index = if len > 0 { index + 1 } else { 0 };
        while next_index < strs[0].len() {
            let next_col: Vec<u8> = strs.iter().map(|s| s.as_bytes()[next_index]).collect();
            if Solution::cols_le(&current_col, &next_col) {
                available_lens.push(Solution::recursion(next_index, len + 1, memo, strs));
            }
            next_index += 1;
        }

        let result = if available_lens.is_empty() {
            len
        } else {
            available_lens.iter().fold(0, |acc, &l| acc.max(l))
        };
        memo[len][index] = Some(result);
        result
    }

    fn cols_le(col1: &[u8], col2: &[u8]) -> bool {
        col1.iter().zip(col2).all(|(c1, c2)| *c1 <= *c2)
    }
}
