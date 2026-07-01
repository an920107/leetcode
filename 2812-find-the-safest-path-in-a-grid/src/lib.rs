pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();

        let mut bfs_queue: VecDeque<BfsState> = VecDeque::new();
        for r in 0..n {
            for c in 0..n {
                if grid[r][c] == 1 {
                    bfs_queue.push_back(BfsState { r, c, d: 0 });
                }
            }
        }

        let mut distance: Vec<Vec<i32>> = vec![vec![i32::MAX; n]; n];
        let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; n];

        while let Some(current) = bfs_queue.pop_front() {
            if visited[current.r][current.c] {
                continue;
            }
            visited[current.r][current.c] = true;
            distance[current.r][current.c] = distance[current.r][current.c].min(current.d);

            if let Some(next_state) = current.get_down(n) {
                bfs_queue.push_back(next_state);
            }
            if let Some(next_state) = current.get_right(n) {
                bfs_queue.push_back(next_state);
            }
            if let Some(next_state) = current.get_up() {
                bfs_queue.push_back(next_state);
            }
            if let Some(next_state) = current.get_left() {
                bfs_queue.push_back(next_state);
            }
        }

        let mut left = 0;
        let mut right = n as i32 * 2;

        while left < right {
            let mid = (left + right) / 2;
            if Self::check_available(&distance, mid) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left - 1
    }

    fn check_available(distance: &Vec<Vec<i32>>, target: i32) -> bool {
        let n = distance.len();
        let mut bfs_queue: VecDeque<BfsState> = VecDeque::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; n];

        if distance[0][0] >= target {
            bfs_queue.push_back(BfsState { r: 0, c: 0, d: 0 });
        }

        while let Some(current) = bfs_queue.pop_front() {
            if visited[current.r][current.c] {
                continue;
            }
            visited[current.r][current.c] = true;

            if current.r == n - 1 && current.c == n - 1 {
                break;
            }

            if let Some(next_state) = current.get_down(n)
                && distance[next_state.r][next_state.c] >= target
            {
                bfs_queue.push_back(next_state);
            }
            if let Some(next_state) = current.get_right(n)
                && distance[next_state.r][next_state.c] >= target
            {
                bfs_queue.push_back(next_state);
            }
            if let Some(next_state) = current.get_up()
                && distance[next_state.r][next_state.c] >= target
            {
                bfs_queue.push_back(next_state);
            }
            if let Some(next_state) = current.get_left()
                && distance[next_state.r][next_state.c] >= target
            {
                bfs_queue.push_back(next_state);
            }
        }

        visited[n - 1][n - 1]
    }
}

#[derive(Clone, Copy)]
struct BfsState {
    r: usize,
    c: usize,
    d: i32,
}

impl BfsState {
    fn get_left(&self) -> Option<BfsState> {
        if self.c > 0 {
            Some(BfsState {
                r: self.r,
                c: self.c - 1,
                d: self.d + 1,
            })
        } else {
            None
        }
    }

    fn get_right(&self, n: usize) -> Option<BfsState> {
        if self.c < n - 1 {
            Some(BfsState {
                r: self.r,
                c: self.c + 1,
                d: self.d + 1,
            })
        } else {
            None
        }
    }

    fn get_up(&self) -> Option<BfsState> {
        if self.r > 0 {
            Some(BfsState {
                r: self.r - 1,
                c: self.c,
                d: self.d + 1,
            })
        } else {
            None
        }
    }

    fn get_down(&self, n: usize) -> Option<BfsState> {
        if self.r < n - 1 {
            Some(BfsState {
                r: self.r + 1,
                c: self.c,
                d: self.d + 1,
            })
        } else {
            None
        }
    }
}
