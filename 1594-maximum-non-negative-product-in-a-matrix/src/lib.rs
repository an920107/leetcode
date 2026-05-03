pub struct Solution;

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut dp: Vec<Vec<(i64, i64)>> = vec![vec![(1, 1); n]; m];

        dp[0][0] = (grid[0][0] as i64, grid[0][0] as i64);
        for i in 1..m {
            let top = dp[i - 1][0];
            let num = grid[i][0] as i64;
            dp[i][0] = if num >= 0 {
                (num * top.0, num * top.1)
            } else {
                (num * top.1, num * top.0)
            };
        }
        for j in 1..n {
            let left = dp[0][j - 1];
            let num = grid[0][j] as i64;
            dp[0][j] = if num >= 0 {
                (num * left.0, num * left.1)
            } else {
                (num * left.1, num * left.0)
            };
        }

        for i in 1..m {
            for j in 1..n {
                let left = dp[i][j - 1];
                let top = dp[i - 1][j];
                let num = grid[i][j] as i64;

                dp[i][j] = if num >= 0 {
                    (num * left.0.max(top.0), num * left.1.min(top.1))
                } else {
                    (num * left.1.min(top.1), num * left.0.max(top.0))
                };
            }
        }

        let result = dp[m - 1][n - 1].0;
        if result >= 0 {
            (result % 1_000_000_007) as i32
        } else {
            -1
        }
    }
}
