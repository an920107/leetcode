pub struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let max_num = nums.iter().max().copied().unwrap() as usize;

        let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; max_num + 1]; max_num + 1]; n + 1];
        dp[0][0][0] = 1;

        for (i, &num) in nums.iter().enumerate() {
            let num = num as usize;
            for j in 0..=max_num {
                let j_gcd = Self::gcd(j, num);
                for k in 0..=max_num {
                    let k_gcd = Self::gcd(k, num);

                    dp[i + 1][j][k] += dp[i][j][k];
                    dp[i + 1][j][k] %= MOD;
                    dp[i + 1][j_gcd][k] += dp[i][j][k];
                    dp[i + 1][j_gcd][k] %= MOD;
                    dp[i + 1][j][k_gcd] += dp[i][j][k];
                    dp[i + 1][j][k_gcd] %= MOD;
                }
            }
        }

        let mut result = 0;
        for seq_gcd in 1..=max_num {
            result += dp[n][seq_gcd][seq_gcd];
            result %= MOD;
        }
        result
    }

    fn gcd(a: usize, b: usize) -> usize {
        if a > b {
            return Self::gcd(b, a);
        }
        if a == 0 { b } else { Self::gcd(b % a, a) }
    }
}
