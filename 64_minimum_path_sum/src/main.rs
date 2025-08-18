fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        for i in 1..rows {
            grid[i][0] += grid[i - 1][0];
        }

        for j in 1..cols {
            grid[0][j] += grid[0][j - 1];
        }

        for i in 1..rows {
            for j in 1..cols {
                grid[i][j] = std::cmp::min(grid[i][j] + grid[i - 1][j], grid[i][j] + grid[i][j - 1])
            }
        }

        grid[rows - 1][cols - 1]
    }
}
