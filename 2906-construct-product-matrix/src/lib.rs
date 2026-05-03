pub struct Solution;

const MOD: i32 = 12345;

impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let size = m * n;
        let flatten_grid: Vec<_> = grid.iter().flatten().copied().collect();

        let mut prefix: Vec<i64> = vec![1; size];
        prefix[0] = (grid[0][0] % MOD) as i64;
        for (index, &num) in flatten_grid.iter().enumerate().skip(1) {
            prefix[index] = prefix[index - 1] * num as i64 % MOD as i64;
        }

        let mut suffix: Vec<i64> = vec![1; size];
        suffix[size - 1] = (grid[m - 1][n - 1] % MOD) as i64;
        for (index, &num) in flatten_grid.iter().enumerate().rev().skip(1) {
            suffix[index] = suffix[index + 1] * num as i64 % MOD as i64;
        }

        let mut result: Vec<Vec<i32>> = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let index = i * n + j;
                let p = if index > 0 { prefix[index - 1] } else { 1 };
                let s = if index < size - 1 {
                    suffix[index + 1]
                } else {
                    1
                    };

                result[i][j] = (p * s % MOD as i64) as i32;
            }
        }

        result
    }
}
