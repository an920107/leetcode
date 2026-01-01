/*
 * @lc app=leetcode id=1970 lang=rust
 *
 * [1970] Last Day Where You Can Still Cross
 */

pub struct Solution;

// @lc code=start
const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Solution {
    pub fn latest_day_to_cross(row_size: i32, col_size: i32, cells: Vec<Vec<i32>>) -> i32 {
        let cells: Vec<(usize, usize)> = cells
            .iter()
            .map(|r| (r[0] as usize, r[1] as usize))
            .collect();

        let mut left = 0usize;
        let mut right = cells.len() - 1;
        while left < right {
            let mid = (left + right) / 2;
            if Solution::can_pass(row_size as usize, col_size as usize, mid, &cells) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left as i32
    }

    fn can_pass(
        row_size: usize,
        col_size: usize,
        index: usize,
        cells: &Vec<(usize, usize)>,
    ) -> bool {
        let mut visited = vec![vec![false; col_size]; row_size];
        let mut grid = vec![vec![false; col_size]; row_size];
        for cell in cells[0..=index].iter() {
            grid[cell.0 - 1][cell.1 - 1] = true;
        }

        for col_index in 0..col_size {
            if grid[0][col_index] {
                continue;
            }
            if Solution::recursion((0, col_index as i32), &mut visited, &grid) {
                return true;
            }
        }

        false
    }

    fn recursion(
        position: (i32, i32),
        visited: &mut Vec<Vec<bool>>,
        grid: &Vec<Vec<bool>>,
    ) -> bool {
        if position.0 < 0
            || position.0 as usize >= grid.len()
            || position.1 < 0
            || position.1 as usize >= grid[0].len()
        {
            return false;
        }

        if visited[position.0 as usize][position.1 as usize] {
            return false;
        }
        visited[position.0 as usize][position.1 as usize] = true;

        // is water
        if grid[position.0 as usize][position.1 as usize] {
            return false;
        }

        if position.0 as usize == grid.len() - 1 {
            return true;
        }

        for direction in DIRECTIONS.iter() {
            if Solution::recursion(
                (position.0 + direction.0, position.1 + direction.1),
                visited,
                grid,
            ) {
                return true;
            }
        }

        false
    }
}
// @lc code=end
