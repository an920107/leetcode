pub struct Solution;

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix[0].len();

        let mut result = 0;
        let mut height: Vec<i32> = vec![0; n];
        for row in matrix.iter() {
            for (index, &bit) in row.iter().enumerate() {
                if bit == 1 {
                    height[index] += 1;
                } else {
                    height[index] = 0;
                }
            }

            let mut sorted_height = height.clone();
            sorted_height.sort_by(|a, b| b.cmp(a));
            result = sorted_height
                .iter()
                .enumerate()
                .map(|(w, h)| (w as i32 + 1) * h)
                .max()
                .unwrap_or(0)
                .max(result);
        }

        result
    }
}
