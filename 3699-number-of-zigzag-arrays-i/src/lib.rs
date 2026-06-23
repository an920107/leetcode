pub struct Solution;

impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        let (n, l, r) = (n as usize, 0usize, (r - l) as usize);

        let mut dp: Vec<Vec<Vec<i64>>> = vec![vec![vec![1; r + 1]; 2]; n + 1];
        let mut prefix: Vec<Vec<Vec<i64>>> = vec![vec![vec![0; r + 2]; 2]; n + 1];

        for i in 0..=r {
            prefix[1][0][i + 1] = prefix[1][0][i] + dp[1][0][i];
            prefix[1][1][i + 1] = prefix[1][1][i] + dp[1][1][i];
        }

        for len in 2..=n {
            for direction in 0..=1 {
                for x in l..=r {
                    let range = if direction == 0 {
                        l..x
                    } else {
                        (x + 1)..(r + 1)
                    };
                    let sum = prefix[len - 1][1 - direction][range.end]
                        - prefix[len - 1][1 - direction][range.start];
                    dp[len][direction][x] = sum % 1_000_000_007;

                    prefix[len][direction][x + 1] =
                        prefix[len][direction][x] + dp[len][direction][x];
                }
            }
        }

        let result =
            (prefix[n][0][r + 1] - prefix[n][0][l]) + (prefix[n][1][r + 1] - prefix[n][1][l]);
        (result % 1_000_000_007) as i32
    }
}
