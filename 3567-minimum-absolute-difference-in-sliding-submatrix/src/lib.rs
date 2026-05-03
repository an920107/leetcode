pub struct Solution;

use std::collections::BTreeSet;

impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let k = k as usize;

        let mut result: Vec<Vec<i32>> = vec![vec![0; n - k + 1]; m - k + 1];

        for i in 0..(m - k + 1) {
            for j in 0..(n - k + 1) {
                let set: BTreeSet<i32> = BTreeSet::from_iter(
                    grid[i..(i + k)]
                        .iter()
                        .map(|row| row[j..(j + k)].to_vec())
                        .flatten(),
                );
                result[i][j] = set
                    .iter()
                    .collect::<Vec<_>>()
                    .windows(2)
                    .map(|s| s[0].abs_diff(*s[1]))
                    .min()
                    .unwrap_or(0) as i32;
            }
        }

        result
    }
}
