pub struct Solution;

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solution {
    pub fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];
        let mut result = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 || visited[i][j] {
                    continue;
                }

                visited[i][j] = true;
                let sum = Self::dfs(&grid, &mut visited, i, j);
                if sum % k as i64 == 0 {
                    result += 1;
                }
            }
        }

        result
    }

    fn dfs(grid: &[Vec<i32>], visited: &mut Vec<Vec<bool>>, i: usize, j: usize) -> i64 {
        let m = grid.len();
        let n = grid[0].len();
        let mut sum = grid[i][j] as i64;

        for &(di, dj) in DIRECTIONS.iter() {
            let (next_i, next_j) = (i as isize + di, j as isize + dj);
            if next_i < 0 || next_j < 0 {
                continue;
            }
            let (next_i, next_j) = (next_i as usize, next_j as usize);
            if next_i as usize >= m || next_j as usize >= n {
                continue;
            }

            if grid[next_i][next_j] == 0 || visited[next_i][next_j] {
                continue;
            }

            visited[next_i][next_j] = true;
            sum += Self::dfs(grid, visited, next_i, next_j);
        }

        sum
    }
}
