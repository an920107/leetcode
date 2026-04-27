pub struct Solution;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut stack = vec![(0, 0)];
        visited[0][0] = true;

        while let Some((r, c)) = stack.pop() {
            if r == m - 1 && c == n - 1 {
                return true;
            }

            let street = grid[r][c];
            let dirs = match street {
                1 => vec![(0, -1), (0, 1)],
                2 => vec![(-1, 0), (1, 0)],
                3 => vec![(0, -1), (1, 0)],
                4 => vec![(0, 1), (1, 0)],
                5 => vec![(0, -1), (-1, 0)],
                6 => vec![(0, 1), (-1, 0)],
                _ => vec![],
            };

            for (dr, dc) in dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if !visited[nr][nc] {
                        let next_street = grid[nr][nc];
                        let next_dirs = match next_street {
                            1 => vec![(0, -1), (0, 1)],
                            2 => vec![(-1, 0), (1, 0)],
                            3 => vec![(0, -1), (1, 0)],
                            4 => vec![(0, 1), (1, 0)],
                            5 => vec![(0, -1), (-1, 0)],
                            6 => vec![(0, 1), (-1, 0)],
                            _ => vec![],
                        };

                        if next_dirs.contains(&(-dr, -dc)) {
                            visited[nr][nc] = true;
                            stack.push((nr, nc));
                        }
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let grid = vec![vec![2, 4, 3], vec![6, 5, 2]];
        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn test_example2() {
        let grid = vec![vec![1, 2, 1], vec![1, 2, 1]];
        assert!(!Solution::has_valid_path(grid));
    }

    #[test]
    fn test_example3() {
        let grid = vec![vec![1, 1, 2]];
        assert!(!Solution::has_valid_path(grid));
    }

    #[test]
    fn test_example4() {
        let grid = vec![vec![1, 1, 1, 1, 1, 1, 3]];
        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn test_example5() {
        let grid = vec![
            vec![2],
            vec![2],
            vec![2],
            vec![2],
            vec![2],
            vec![2],
            vec![6],
        ];
        assert!(Solution::has_valid_path(grid));
    }
}
