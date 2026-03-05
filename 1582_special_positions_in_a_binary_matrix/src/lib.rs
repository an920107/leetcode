pub struct Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        let mut result = 0;

        for (i, row) in mat.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                if val == 0 {
                    continue;
                }

                if (0..j).map(|k| row[k]).any(|v| v == 1)
                    || ((j + 1)..n).map(|k| row[k]).any(|v| v == 1)
                    || (0..i).map(|k| mat[k][j]).any(|v| v == 1)
                    || ((i + 1)..m).map(|k| mat[k][j]).any(|v| v == 1)
                {
                    continue;
                }

                result += 1;
            }
        }

        result
    }
}
