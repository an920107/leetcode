/*
 * @lc app=leetcode id=3651 lang=rust
 *
 * [3651] Minimum Cost Path with Teleportations
 */

pub struct Solution;

// @lc code=start
use std::collections::{BTreeMap, HashMap};

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut vals: BTreeMap<i32, Vec<(usize, usize)>> = BTreeMap::new();
        for (i, row) in grid.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                vals.entry(val)
                    .and_modify(|m| m.push((i, j)))
                    .or_insert(vec![(i, j)]);
            }
        }

        let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![i32::MAX; n]; m]; k as usize + 1];
        for l in 0..=k as usize {
            dp[l][0][0] = 0;
        }
        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    continue;
                }
                let current_val = grid[i][j];
                if i > 0 {
                    dp[0][i][j] = dp[0][i][j].min(dp[0][i - 1][j] + current_val);
                }
                if j > 0 {
                    dp[0][i][j] = dp[0][i][j].min(dp[0][i][j - 1] + current_val);
                }
            }
        }

        for l in 1..=k as usize {
            let mut min_teleport_cost_by_val: HashMap<i32, i32> = HashMap::new();
            let mut min_cost = i32::MAX;
            for (&val, poss) in vals.iter().rev() {
                for &pos in poss.iter() {
                    min_cost = min_cost.min(dp[l - 1][pos.0][pos.1]);
                }
                min_teleport_cost_by_val.insert(val, min_cost);
            }

            for i in 0..m {
                for j in 0..n {
                    if i == 0 && j == 0 {
                        continue;
                    }

                    let current_val = grid[i][j];
                    if i > 0 {
                        dp[l][i][j] = dp[l][i][j].min(dp[l][i - 1][j] + current_val);
                    }
                    if j > 0 {
                        dp[l][i][j] = dp[l][i][j].min(dp[l][i][j - 1] + current_val);
                    }
                    if let Some(&cost) = min_teleport_cost_by_val.get(&current_val) {
                        dp[l][i][j] = dp[l][i][j].min(cost);
                    }
                }
            }
        }

        dp.iter().map(|costs| costs[m - 1][n - 1]).min().unwrap()
    }
}
// @lc code=end
