pub struct Solution;

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];
        let mut result = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 || visited[i][j] {
                    continue;
                }
                if Self::dfs(&grid, &mut visited, i, j) {
                    result += 1;
                }
            }
        }

        result
    }

    fn dfs(grid: &[Vec<i32>], visited: &mut Vec<Vec<bool>>, i: usize, j: usize) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        let mut result = true;

        for &(di, dj) in DIRECTIONS.iter() {
            let next_i = i as isize + di;
            let next_j = j as isize + dj;

            if next_i < 0 || next_j < 0 || next_i as usize >= m || next_j as usize >= n {
                result = false;
                continue;
            }

            let next_i = next_i as usize;
            let next_j = next_j as usize;

            if visited[next_i][next_j] || grid[next_i][next_j] == 1 {
                continue;
            }
            visited[next_i as usize][next_j as usize] = true;
            if !Self::dfs(grid, visited, next_i, next_j) {
                result = false;
            }
        }

        result
    }
}
