pub struct Solution;

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        if m == 1 && n == 1 {
            return false;
        }

        let mut row_prefix: Vec<i64> = vec![0; m + 1];
        for (i, row) in grid.iter().enumerate() {
            row_prefix[i + 1] = row_prefix[i] + row.iter().map(|n| *n as i64).sum::<i64>()
        }
        for h_cut in 1..m {
            let top = row_prefix[h_cut];
            let bottom = row_prefix[m] - row_prefix[h_cut];
            if top == bottom {
                return true;
            }
        }

        let mut col_prefix: Vec<i64> = vec![0; n + 1];
        for j in 0..n {
            let mut sum = 0i64;
            for i in 0..m {
                sum += grid[i][j] as i64;
            }
            col_prefix[j + 1] = col_prefix[j] + sum;
        }
        for v_cut in 1..n {
            let left = col_prefix[v_cut];
            let right = col_prefix[n] - col_prefix[v_cut];
            if left == right {
                return true;
            }
        }

        false
    }
}
