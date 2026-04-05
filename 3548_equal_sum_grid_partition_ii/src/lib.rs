pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let mut grid: Vec<Vec<i64>> = grid
            .into_iter()
            .map(|row| row.into_iter().map(|num| num as i64).collect())
            .collect();

        let sum: i64 = grid.iter().flatten().sum();

        for _ in 0..4 {
            let m = grid.len();
            let n = grid[0].len();

            let mut seen_nums: HashSet<i64> = HashSet::new();
            let mut current_sum = Solution::count_row_sum(&grid[0], &mut seen_nums);

            for i in 0..(m - 1) {
                let other_part_sum = sum - current_sum;
                let diff = current_sum - other_part_sum;

                if diff == 0 {
                    return true;
                }

                if seen_nums.contains(&diff) {
                    if i > 0 && n > 1 {
                        return true;
                    }

                    if n > 1 && (grid[i][0] == diff || grid[i][n - 1] == diff) {
                        return true;
                    }

                    if n == 1 && (grid[0][0] == diff || grid[i][0] == diff) {
                        return true;
                    }
                }

                current_sum += Solution::count_row_sum(&grid[i + 1], &mut seen_nums);
            }

            grid = Solution::rotate(&grid);
        }

        false
    }

    fn count_row_sum(row: &Vec<i64>, seen: &mut HashSet<i64>) -> i64 {
        let mut sum = 0;
        for num in row.iter() {
            sum += *num;
            seen.insert(*num);
        }
        sum
    }

    fn rotate<T: Copy + Default>(mat: &Vec<Vec<T>>) -> Vec<Vec<T>> {
        let m = mat.len();
        let n = mat[0].len();

        let mut new_mat = vec![vec![T::default(); m]; n];

        for i in 0..m {
            for j in 0..n {
                new_mat[j][m - i - 1] = mat[i][j];
            }
        }

        new_mat
    }
}
