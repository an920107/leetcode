pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[0][0] == 1 {
            return 0;
        }

        let m = obstacle_grid.len();
        let n = obstacle_grid.first().unwrap().len();

        let mut grid = vec![0; n];
        grid[0] = 1;

        for i in 0..m {
            for j in 0..n {
                if obstacle_grid[i][j] == 1 {
                    grid[j] = 0;
                } else {
                    if j > 0 {
                        grid[j] += grid[j - 1];
                    }
                }
            }
        }

        *grid.last().unwrap()
    }
}
