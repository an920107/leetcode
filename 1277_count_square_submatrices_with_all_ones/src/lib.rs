pub struct Solution;

impl Solution {
    pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut result = 0;

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 1 {
                    if i > 0 && j > 0 {
                        let available_size =
                            *[matrix[i - 1][j - 1], matrix[i - 1][j], matrix[i][j - 1]]
                                .iter()
                                .min()
                                .unwrap();
                        matrix[i][j] += available_size;
                    }

                    result += matrix[i][j];
                }
            }
        }

        result
    }
}
