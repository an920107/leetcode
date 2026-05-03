pub struct Solution;

impl Solution {
    pub fn num_submat(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut result = 0;
        let mut heights = vec![0; n];
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 1 {
                    heights[j] += matrix[i][j];
                } else {
                    heights[j] = 0;
                }
            }

            let mut index_stack: Vec<usize> = vec![];
            let mut previos_less_index = vec![-1 as isize; n];
            for j in 0..n {
                while let Some(&index) = index_stack.last() {
                    if heights[j] > heights[index] {
                        previos_less_index[j] = index as isize;
                        break;
                    } else {
                        index_stack.pop();
                    }
                }
                index_stack.push(j);
            }

            let mut sum = vec![0; n];
            for j in 0..n {
                if previos_less_index[j] == -1 {
                    sum[j] = heights[j] * (j as i32 + 1);
                } else {
                    sum[j] = sum[previos_less_index[j] as usize]
                        + heights[j] * (j as i32 - previos_less_index[j] as i32);
                }

                result += sum[j];
            }
        }

        result
    }
}
