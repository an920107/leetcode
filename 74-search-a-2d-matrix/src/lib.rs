pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix.first().unwrap().len();

        let mut lower = 0;
        let mut upper = m * n;
        while lower < upper {
            let mid = (lower + upper) / 2;
            let num = matrix[mid / n][mid % n];
            if num > target {
                upper = mid;
            } else if num < target {
                lower = mid + 1;
            } else {
                return true;
            }
        }

        false
    }
}
