pub struct Solution;

impl Solution {
    pub fn find_rotation(mut mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        if Solution::is_matrix_equal(&mat, &target) {
            return true;
        }
        for _ in 0..3 {
            Solution::rotate_counterclockwise(&mut mat);
            if Solution::is_matrix_equal(&mat, &target) {
                return true;
            }
        }

        false
    }

    fn is_matrix_equal(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> bool {
        let n = a.len();

        for i in 0..n {
            for j in 0..n {
                if a[i][j] != b[i][j] {
                    return false;
                }
            }
        }

        true
    }

    fn rotate_counterclockwise(mat: &mut Vec<Vec<i32>>) {
        let n = mat.len();

        for i in 1..n {
            for j in 0..i {
                let temp = mat[i][j];
                mat[i][j] = mat[j][i];
                mat[j][i] = temp;
            }
        }

        mat.reverse();
    }
}
