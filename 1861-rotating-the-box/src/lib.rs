pub struct Solution;

impl Solution {
    pub fn rotate_the_box(mut box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let (m, n) = (box_grid.len(), box_grid[0].len());

        for row in box_grid.iter_mut() {
            let mut index = 0;
            for part in row.clone().split(|&c| c == '*') {
                let empty_count = part.iter().filter(|&c| *c == '.').count();

                for offset in 0..empty_count {
                    row[index + offset] = '.';
                }
                for offset in empty_count..part.len() {
                    row[index + offset] = '#';
                }

                index += part.len() + 1;
            }
        }

        let mut rotated_box_grid = vec![vec!['\0'; m]; n];
        let (n, m) = (m, n);
        for i in 0..m {
            for j in 0..n {
                rotated_box_grid[i][j] = box_grid[n - 1 - j][i]
            }
        }
        rotated_box_grid
    }
}
