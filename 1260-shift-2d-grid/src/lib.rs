pub struct Solution;

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let col_count = grid[0].len();
        let flatten_grid: Vec<i32> = grid.iter().flatten().copied().collect();
        let n = flatten_grid.len();
        let k = (k as usize) % n;
        let (left, right) = flatten_grid.split_at(n - k);
        let moved_flatten_grid = [right, left].concat();
        moved_flatten_grid
            .chunks(col_count)
            .map(|v| v.to_vec())
            .collect()
    }
}
