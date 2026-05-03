pub struct Solution;

const MIN: i32 = -100_000_000;

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let m = coins.len();
        let n = coins[0].len();

        let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![MIN; 3]; n + 1]; m + 1];
        dp[0][1][2] = 0;

        for i in 1..=m {
            for j in 1..=n {
                let current_coin = coins[i - 1][j - 1];
                dp[i][j][2] = dp[i - 1][j][2].max(dp[i][j - 1][2]) + current_coin;
                for k in (0..2).rev() {
                    dp[i][j][k] = (dp[i - 1][j][k].max(dp[i][j - 1][k]) + current_coin)
                        .max(dp[i - 1][j][k + 1].max(dp[i][j - 1][k + 1]))
                }
            }
        }

        *dp[m][n].iter().max().unwrap()
    }
}
