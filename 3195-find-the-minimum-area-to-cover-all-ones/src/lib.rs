pub struct Solution;

impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let top = (|| {
            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == 1 {
                        return i;
                    }
                }
            }
            m - 1
        })();

        let left = (|| {
            for j in 0..n {
                for i in top..m {
                    if grid[i][j] == 1 {
                        return j;
                    }
                }
            }
            n - 1
        })();

        let bottom = (|| {
            for i in (top..m).rev() {
                for j in left..n {
                    if grid[i][j] == 1 {
                        return i;
                    }
                }
            }
            top
        })();

        let right = (|| {
            for j in (left..n).rev() {
                for i in top..(bottom + 1) {
                    if grid[i][j] == 1 {
                        return j;
                    }
                }
            }
            left
        })();

        ((bottom - top + 1) * (right - left + 1)) as i32
    }
}
