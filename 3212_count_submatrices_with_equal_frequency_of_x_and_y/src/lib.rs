pub struct Solution;

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut sums: Vec<Vec<Count>> = vec![vec![Count::new(); n + 1]; m + 1];

        for i in 0..m {
            for j in 0..n {
                let top = sums[i][j + 1];
                let left = sums[i + 1][j];
                let left_top = sums[i][j];
                sums[i + 1][j + 1] = top + left - left_top
                    + match grid[i][j] {
                        'X' => Count { x: 1, y: 0 },
                        'Y' => Count { x: 0, y: 1 },
                        _ => Count::new(),
                    };
            }
        }

        sums.into_iter()
            .flatten()
            .filter(|c| c.x > 0 && c.x == c.y)
            .count() as i32
    }
}

#[derive(Clone, Copy)]
struct Count {
    x: i32,
    y: i32,
}

impl Count {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl std::ops::Add for Count {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Count {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
