pub struct Solution;

const MOD: i32 = 1000000007;

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let zero = zero as usize;
        let one = one as usize;
        let limit = limit as usize;

        let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; 2]; one + 1]; zero + 1];
        for i in 0..=zero.min(limit) {
            dp[i][0][0] = 1;
        }
        for j in 0..=one.min(limit) {
            dp[0][j][1] = 1;
        }

        for i in 1..=zero {
            for j in 1..=one {
                if i > limit {
                    dp[i][j][0] = dp[i - 1][j][0] + dp[i - 1][j][1] - dp[i - (limit + 1)][j][1];
                } else {
                    dp[i][j][0] = dp[i - 1][j][0] + dp[i - 1][j][1];
                }
                dp[i][j][0] = (dp[i][j][0] % MOD + MOD) % MOD;

                if j > limit {
                    dp[i][j][1] = dp[i][j - 1][0] + dp[i][j - 1][1] - dp[i][j - (limit + 1)][0];
                } else {
                    dp[i][j][1] = dp[i][j - 1][0] + dp[i][j - 1][1];
                }
                dp[i][j][1] = (dp[i][j][1] % MOD + MOD) % MOD;
            }
        }

        (dp[zero][one][0] + dp[zero][one][1]) % MOD
    }
}
