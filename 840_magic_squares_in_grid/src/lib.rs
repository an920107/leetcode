/*
 * @lc app=leetcode id=840 lang=rust
 *
 * [840] Magic Squares In Grid
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        if m < 3 || n < 3 {
            return 0;
        }

        let mut result = 0;

        for row_index in 0..(m - 2) {
            for col_index in 0..(n - 2) {
                let subgrid: Vec<Vec<i32>> = grid[row_index..(row_index + 3)]
                    .iter()
                    .map(|row| {
                        row[col_index..(col_index + 3)]
                            .iter()
                            .copied()
                            .collect::<Vec<i32>>()
                    })
                    .collect();
                if Solution::is_magic_grid(&subgrid) {
                    result += 1;
                }
            }
        }

        result
    }

    fn is_magic_grid(grid: &Vec<Vec<i32>>) -> bool {
        let mut pool: Vec<bool> = vec![false; 9];
        for row in grid.iter() {
            for &num in row {
                if num < 1 || num > 9 {
                    return false;
                }
                pool[num as usize - 1] = true;
            }
        }
        if pool.iter().any(|b| !*b) {
            return false;
        }

        for row in grid.iter() {
            if row.iter().sum::<i32>() != 15 {
                return false;
            }
        }

        for col_index in 0..3 {
            let mut col_sum = 0;
            for row_index in 0..3 {
                col_sum += grid[row_index][col_index];
            }
            if col_sum != 15 {
                return false;
            }
        }

        if grid[0][0] + grid[1][1] + grid[2][2] != 15 {
            return false;
        }

        if grid[0][2] + grid[1][1] + grid[2][0] != 15 {
            return false;
        }

        true
    }
}
// @lc code=end
