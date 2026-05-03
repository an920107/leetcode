pub struct Solution;

impl Solution {
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut sums: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];

        for i in 0..m {
            for j in 0..n {
                let top = sums[i][j + 1];
                let left = sums[i + 1][j];
                let left_top = sums[i][j];
                sums[i + 1][j + 1] = grid[i][j] + top + left - left_top;
            }
        }

        (sums.into_iter().flatten().filter(|&n| n <= k).count() - m - n - 1) as i32
    }
}
