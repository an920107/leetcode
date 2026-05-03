/*
 * @lc app=leetcode id=1292 lang=rust
 *
 * [1292] Maximum Side Length of a Square with Sum Less than or Equal to Threshold
 */

pub struct Solution;

// @lc code=start
impl Solution {

    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        let mut row_prefix = vec![vec![0; n + 1]; m];
        for (i, row) in mat.iter().enumerate() {
            for (j, num) in row.iter().enumerate() {
                row_prefix[i][j + 1] = row_prefix[i][j] + num;
            }
        }

        let mut prefix = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                prefix[i][j] = row_prefix[i - 1][j] + prefix[i - 1][j];
            }
        }

        let mut left = 0usize;
        let mut right = m.min(n) + 1;
        while left < right {
            let mid = (left + right) / 2;
            let mut le_threshold = false;

            'outer: for i in 0..=(m - mid) {
                for j in 0..=(n - mid) {
                    if Solution::sum_of_square((i, j), mid, &prefix) <= threshold {
                        le_threshold = true;
                        break 'outer;
                    }
                }
            }

            if le_threshold {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left as i32 - 1
    }

    fn sum_of_square(pos: (usize, usize), side_len: usize, prefix: &Vec<Vec<i32>>) -> i32 {
        prefix[pos.0][pos.1] + prefix[pos.0 + side_len][pos.1 + side_len]
            - prefix[pos.0 + side_len][pos.1]
            - prefix[pos.0][pos.1 + side_len]
    }
}
// @lc code=end
