pub struct Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let mut prefix: Vec<i32> = vec![0; piles.len() + 1];
        for (index, &pile) in piles.iter().enumerate() {
            prefix[index + 1] = prefix[index] + pile;
        }
        let mut memo = vec![vec![vec![None; prefix.len()]; prefix.len()]; 2];
        Self::max_number_of_stones(&prefix, &mut memo, 0, 0, 1).0
    }

    fn max_number_of_stones(
        prefix: &[i32],
        memo: &mut Vec<Vec<Vec<Option<(i32, i32)>>>>,
        turn: usize,
        index: usize,
        m: usize,
    ) -> (i32, i32) {
        if index >= prefix.len() - 1 {
            return (0, 0);
        }

        if let Some(result) = memo[turn][index][m] {
            return result;
        }

        let mut result = (0, 0);
        for k in 1..=(m * 2) {
            if index + k >= prefix.len() {
                break;
            }
            let base =
                Self::max_number_of_stones(prefix, memo, (turn + 1) % 2, index + k, m.max(k));
            let range_sum = prefix[index + k] - prefix[index];
            if turn % 2 == 0 {
                if base.0 + range_sum > result.0 {
                    result = (base.0 + range_sum, base.1)
                }
            } else {
                if base.1 + range_sum > result.1 {
                    result = (base.0, base.1 + range_sum);
                }
            }
        }

        memo[turn][index][m] = Some(result);
        result
    }
}

#[test]
fn test_solution() {
    Solution::stone_game_ii(vec![2, 7, 9, 4, 4]);
}
