pub struct Solution;

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid.first().unwrap().len();

        let mut initial_visited: Vec<Vec<bool>> =
            vec![vec![false; grid.first().unwrap().len()]; grid.len()];

        let mut start_point: (i32, i32) = (0, 0);
        let mut end_point: (i32, i32) = (0, 0);
        let mut left_blocks = (m * n) as i32;
        for (i, inner_vec) in grid.iter().enumerate() {
            for (j, &value) in inner_vec.iter().enumerate() {
                match value {
                    1 => start_point = (i as i32, j as i32),
                    2 => end_point = (i as i32, j as i32),
                    -1 => {
                        initial_visited[i][j] = true;
                        left_blocks -= 1;
                    }
                    _ => {}
                }
            }
        }

        let mut dfs = DFS {
            directions: vec![(1, 0), (0, 1), (-1, 0), (0, -1)],
            end_point: end_point,
            result: 0,
        };
        dfs.run(DFSParams {
            point: start_point,
            visited: initial_visited,
            left_blocks: left_blocks - 1,
        });
        dfs.result
    }
}

struct DFS {
    directions: Vec<(i32, i32)>,
    end_point: (i32, i32),
    result: i32,
}

struct DFSParams {
    point: (i32, i32),
    visited: Vec<Vec<bool>>,
    left_blocks: i32,
}

impl DFS {
    fn run(&mut self, params: DFSParams) {
        let mut stack: Vec<DFSParams> = vec![params];

        while !stack.is_empty() {
            let current = stack.pop().unwrap();

            if current.point.0 < 0
                || current.point.1 < 0
                || current.point.0 >= current.visited.len() as i32
                || current.point.1 >= current.visited.first().unwrap().len() as i32
                || current.visited[current.point.0 as usize][current.point.1 as usize]
            {
                continue;
            }

            if current.point == self.end_point {
                if current.left_blocks == 0 {
                    self.result += 1;
                }
            }

            for &direction in &self.directions {
                let mut new_visited = current.visited.clone();
                new_visited[current.point.0 as usize][current.point.1 as usize] = true;

                stack.push(DFSParams {
                    point: (current.point.0 + direction.0, current.point.1 + direction.1),
                    visited: new_visited,
                    left_blocks: current.left_blocks - 1,
                });
            }
        }
    }
}
