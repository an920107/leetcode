pub struct Solution;

use std::collections::VecDeque;

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        let mut costs: Vec<Vec<i32>> = vec![vec![i32::MAX; n]; m];
        costs[0][0] = grid[0][0];

        let mut bfs_queue: VecDeque<(usize, usize)> = VecDeque::new();
        bfs_queue.push_back((0, 0));

        while let Some(current_pos) = bfs_queue.pop_front() {
            if current_pos == (m - 1, n - 1) {
                return true;
            }

            for direction in DIRECTIONS {
                let next_pos = (
                    current_pos.0 as i32 + direction.0,
                    current_pos.1 as i32 + direction.1,
                );
                if next_pos.0 < 0
                    || next_pos.1 < 0
                    || next_pos.0 as usize >= m
                    || next_pos.1 as usize >= n
                {
                    continue;
                }

                let next_pos = (next_pos.0 as usize, next_pos.1 as usize);
                let next_cost = costs[current_pos.0][current_pos.1] + grid[next_pos.0][next_pos.1];
                if next_cost >= health || next_cost >= costs[next_pos.0][next_pos.1] {
                    continue;
                }

                costs[next_pos.0][next_pos.1] = next_cost;
                if grid[next_pos.0][next_pos.1] == 0 {
                    bfs_queue.push_front(next_pos);
                } else {
                    bfs_queue.push_back(next_pos);
                }
            }
        }

        false
    }
}
