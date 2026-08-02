pub struct Solution;

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let n = piles.len();
        let mut memo = vec![vec![None; n]; n];
        Self::max_scores(&piles, &mut memo, 0, n - 1) > 0
    }

    fn max_scores(piles: &[i32], memo: &mut Vec<Vec<Option<i32>>>, l: usize, r: usize) -> i32 {
        if let Some(result) = memo[l][r] {
            return result;
        }

        if l == r {
            let result = piles[l];
            memo[l][r] = Some(result);
            return result;
        }

        let select_left = piles[l] - Self::max_scores(piles, memo, l + 1, r);
        let select_right = piles[r] - Self::max_scores(piles, memo, l, r - 1);

        let result = select_left.max(select_right);
        memo[l][r] = Some(result);
        result
    }
}
