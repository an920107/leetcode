pub struct Solution;

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_col: i32) -> f64 {
        let query_row = query_row as usize;
        let query_col = query_col as usize;

        let mut cups: Vec<Vec<Option<f64>>> = vec![vec![None; query_row + 2]; query_row + 1];
        cups[0][1] = Some(poured as f64);

        for row in 1..=query_row {
            for col in 1..=(row + 1) {
                let left = cups[row - 1]
                    .get(col - 1)
                    .copied()
                    .flatten()
                    .map_or(0., |l| ((l - 1.) / 2.).max(0.));
                let right = cups[row - 1]
                    .get(col)
                    .copied()
                    .flatten()
                    .map_or(0., |l| ((l - 1.) / 2.).max(0.));
                cups[row][col] = Some(left + right);
            }
        }

        cups[query_row][query_col + 1].unwrap().min(1.)
    }
}
