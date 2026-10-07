pub struct Solution;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        if grid[0][0] == ')' {
            return false;
        }
        if (m + n - 1) % 2 == 1 {
            return false;
        }

        let mut memo = vec![vec![vec![None; (m + n) / 2 + 1]; n]; m];

        Self::dfs(&grid, &mut memo, 0, 0, 1)
    }

    fn dfs(
        grid: &Vec<Vec<char>>,
        memo: &mut Vec<Vec<Vec<Option<bool>>>>,
        row_index: usize,
        col_index: usize,
        depth: usize,
    ) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        if depth > (m + n) / 2 {
            return false;
        }

        if row_index == m - 1 && col_index == n - 1 {
            return depth == 0;
        }

        if let Some(result) = memo[row_index][col_index][depth] {
            return result;
        }

        if col_index < n - 1 {
            let next_c = grid[row_index][col_index + 1];
            if next_c == '(' && Self::dfs(grid, memo, row_index, col_index + 1, depth + 1) {
                return true;
            }
            if next_c == ')'
                && depth > 0
                && Self::dfs(grid, memo, row_index, col_index + 1, depth - 1)
            {
                return true;
            }
        }

        if row_index < m - 1 {
            let next_c = grid[row_index + 1][col_index];
            if next_c == '(' && Self::dfs(grid, memo, row_index + 1, col_index, depth + 1) {
                return true;
            }
            if next_c == ')'
                && depth > 0
                && Self::dfs(grid, memo, row_index + 1, col_index, depth - 1)
            {
                return true;
            }
        }

        memo[row_index][col_index][depth] = Some(false);
        false
    }
}
