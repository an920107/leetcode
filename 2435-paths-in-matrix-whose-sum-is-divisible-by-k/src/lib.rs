pub struct Solution;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; k as usize]; n]; m];

        dp[0][0][(grid[0][0] % k) as usize] = 1;

        for row in 1..m {
            let num = grid[row][0];
            let mut rs = vec![0; k as usize];

            let prev = &dp[row - 1][0];
            let mut r = 0usize;
            while r < k as usize {
                let count = prev[r];
                rs[((r as i32 + num) % k) as usize] = count % 1_000_000_007;
                r += 1;
            }

            dp[row][0] = rs;
        }
        for col in 1..n {
            let num = grid[0][col];
            let mut rs = vec![0; k as usize];

            let prev = &dp[0][col - 1];
            let mut r = 0usize;
            while r < k as usize {
                let count = prev[r];
                rs[((r as i32 + num) % k) as usize] = count % 1_000_000_007;
                r += 1;
            }

            dp[0][col] = rs;
        }

        for row in 1..m {
            for col in 1..n {
                let num = grid[row][col];
                let mut rs = vec![0; k as usize];

                let prev1 = &dp[row - 1][col];
                let prev2 = &dp[row][col - 1];
                let mut r = 0usize;
                while r < k as usize {
                    let count1 = prev1[r];
                    let count2 = prev2[r];
                    rs[((r as i32 + num) % k) as usize] = (count1 + count2) % 1_000_000_007;
                    r += 1;
                }

                dp[row][col] = rs;
            }
        }

        dp[m - 1][n - 1][0]
    }
}

/*

5 2 4
3 0 5
0 7 2

[0,0,1] [0,0,1] [0,1,0]
[1,0,0] [1,0,1] [1,1,1]
[1,0,0] []

*/
