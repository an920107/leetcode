pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        if k < 0 {
            return -1;
        }

        let m = grid.len();
        let n = grid[0].len();
        let max_k = (k as usize).min(m + n);

        let mut dp = vec![vec![vec![None; max_k + 1]; n]; m];
        dp[0][0][0] = Some(grid[0][0]);

        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    continue;
                }

                let cost = Self::cell_to_cost(grid[i][j]) as usize;

                for c in cost..=max_k {
                    let mut max_val: Option<i32> = None;

                    if i > 0 {
                        if let Some(val) = dp[i - 1][j][c - cost] {
                            max_val = Some(max_val.map_or(val, |v| v.max(val)));
                        }
                    }

                    if j > 0 {
                        if let Some(val) = dp[i][j - 1][c - cost] {
                            max_val = Some(max_val.map_or(val, |v| v.max(val)));
                        }
                    }

                    if let Some(val) = max_val {
                        dp[i][j][c] = Some(val + grid[i][j]);
                    }
                }
            }
        }

        let mut ans: Option<i32> = None;
        for c in 0..=max_k {
            if let Some(val) = dp[m - 1][n - 1][c] {
                ans = Some(ans.map_or(val, |v| v.max(val)));
            }
        }

        ans.unwrap_or(-1)
    }

    fn recursion(
        grid: &Vec<Vec<i32>>,
        memo: &mut HashMap<(usize, usize, i32), Option<i32>>,
        pos: (usize, usize),
        k: i32,
    ) -> Option<i32> {
        if k < 0 {
            return None;
        }

        let m = grid.len();
        let n = grid[0].len();

        if pos == (m - 1, n - 1) {
            return Some(grid[pos.0][pos.1]);
        }

        if let Some(&result) = memo.get(&(pos.0, pos.1, k)) {
            return result;
        }

        let right_pos = (pos.0, pos.1 + 1);
        let right = if right_pos.1 < n {
            Self::recursion(
                grid,
                memo,
                right_pos,
                k - Self::cell_to_cost(grid[right_pos.0][right_pos.1]),
            )
        } else {
            None
        };

        let down_pos = (pos.0 + 1, pos.1);
        let down = if down_pos.0 < m {
            Self::recursion(
                grid,
                memo,
                down_pos,
                k - Self::cell_to_cost(grid[down_pos.0][down_pos.1]),
            )
        } else {
            None
        };

        let result = match (right, down) {
            (None, None) => None,
            (Some(val), None) | (None, Some(val)) => Some(val + grid[pos.0][pos.1]),
            (Some(val1), Some(val2)) => Some(val1.max(val2) + grid[pos.0][pos.1]),
        };

        memo.insert((pos.0, pos.1, k), result);
        return result;
    }

    fn cell_to_cost(cell: i32) -> i32 {
        match cell {
            0 => 0,
            1 | 2 => 1,
            _ => unreachable!(),
        }
    }
}
