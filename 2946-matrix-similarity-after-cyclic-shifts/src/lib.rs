pub struct Solution;

impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let n = mat[0].len();
        let k = n % k as usize;

        let mut shifted_mat = mat.clone();

        shifted_mat = shifted_mat
            .into_iter()
            .enumerate()
            .map(|(index, row)| {
                if index % 2 == 0 {
                    Solution::shift_left(&row, k)
                } else {
                    Solution::shift_right(&row, k)
                }
            })
            .collect();

        Solution::check_equal(&shifted_mat, &mat)
    }

    fn shift_left(row: &Vec<i32>, step: usize) -> Vec<i32> {
        let n = row.len();
        let mut right = row[0..step].to_vec();
        let mut left = row[step..n].to_vec();
        left.append(&mut right);
        left
    }

    fn shift_right(row: &Vec<i32>, step: usize) -> Vec<i32> {
        let n = row.len();
        let mut right = row[0..(n - step)].to_vec();
        let mut left = row[(n - step)..n].to_vec();
        left.append(&mut right);
        left
    }

    fn check_equal(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> bool {
        a.iter()
            .flatten()
            .zip(b.iter().flatten())
            .all(|(ai, bi)| ai == bi)
    }
}
