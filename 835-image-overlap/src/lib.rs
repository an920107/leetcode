pub struct Solution;

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len();

        let mut canvas: Vec<Vec<i32>> = vec![vec![0; n * 3 - 2]; n * 3 - 2];
        for (row_index, row) in img1.iter().enumerate() {
            for (col_index, bit) in row.iter().enumerate() {
                canvas[row_index + n - 1][col_index + n - 1] = *bit;
            }
        }

        let mut result = 0;

        for row_offset in 0..(n * 2 - 1) {
            for col_offset in 0..(n * 2 - 1) {
                let iter = canvas[row_offset..(row_offset + n)]
                    .iter()
                    .map(|row| &row[col_offset..(col_offset + n)]);
                result = result.max(Self::count_overlap(iter, img2.iter().map(|row| row)))
            }
        }

        result
    }

    fn count_overlap<'a>(
        img1: impl IntoIterator<Item = impl IntoIterator<Item = &'a i32>>,
        img2: impl IntoIterator<Item = impl IntoIterator<Item = &'a i32>>,
    ) -> i32 {
        let mut count = 0;
        for (row1, row2) in img1.into_iter().zip(img2.into_iter()) {
            for (bit1, bit2) in row1.into_iter().zip(row2.into_iter()) {
                count += bit1 & bit2;
            }
        }
        count
    }
}
