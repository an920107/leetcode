/*
 * @lc app=leetcode id=1895 lang=rust
 *
 * [1895] Largest Magic Square
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut row_prefix = vec![vec![0; n + 1]; m];
        let mut col_prefix = vec![vec![0; m + 1]; n];

        for (i, row) in grid.iter().enumerate() {
            for (j, num) in row.iter().enumerate() {
                row_prefix[i][j + 1] = row_prefix[i][j] + num;
            }
        }
        for i in 0..n {
            for j in 0..m {
                let num = grid[j][i];
                col_prefix[i][j + 1] = col_prefix[i][j] + num;
            }
        }

        let mut result = 1;
        for length in 2..=m.min(n) {
            for i in 0..=(m - length) {
                for j in 0..=(n - length) {
                    if Solution::is_magic_square((i, j), length, &grid, &row_prefix, &col_prefix) {
                        result = result.max(length);
                    }
                }
            }
        }

        result as i32
    }

    fn is_magic_square(
        pos: (usize, usize),
        length: usize,
        grid: &Vec<Vec<i32>>,
        row_prefix: &Vec<Vec<i32>>,
        col_prefix: &Vec<Vec<i32>>,
    ) -> bool {
        let sum = row_prefix[pos.0][pos.1 + length] - row_prefix[pos.0][pos.1];

        for i in pos.0..(pos.0 + length) {
            let row_sum = row_prefix[i][pos.1 + length] - row_prefix[i][pos.1];
            if sum != row_sum {
                return false;
            }
        }

        for i in pos.1..(pos.1 + length) {
            let col_sum = col_prefix[i][pos.0 + length] - col_prefix[i][pos.0];
            if sum != col_sum {
                return false;
            }
        }

        let mut dia_sum = 0;
        for offset in 0..length {
            dia_sum += grid[pos.0 + offset][pos.1 + offset];
        }
        if sum != dia_sum {
            return false;
        }

        let mut dia_sum = 0;
        for offset in 0..length {
            dia_sum += grid[pos.0 + offset][pos.1 + length - offset - 1];
        }
        if sum != dia_sum {
            return false;
        }

        true
    }
}
// @lc code=end
