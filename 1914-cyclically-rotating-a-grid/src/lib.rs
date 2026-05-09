pub struct Solution;

impl Solution {
    pub fn rotate_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let perimeter = (m + n - 2) * 2;

        let mut layers: Vec<Vec<i32>> = vec![];

        for l in 0..(m.min(n) / 2) {
            layers.push(vec![]);
            let mut pos = Position(l, l);
            let mut direction = Direction::Right;
            for _ in 0..(perimeter - 8 * l) {
                match direction {
                    Direction::Right => {
                        if pos.1 == n - l - 1 {
                            direction = Direction::Down;
                        }
                    }
                    Direction::Down => {
                        if pos.0 == m - l - 1 {
                            direction = Direction::Left;
                        }
                    }
                    Direction::Left => {
                        if pos.1 == l {
                            direction = Direction::Up;
                        }
                    }
                    Direction::Up => {}
                }
                layers.last_mut().unwrap().push(grid[pos.0][pos.1]);
                pos += direction;
            }
        }

        for layer in layers.iter_mut() {
            let k = k as usize % layer.len();
            let mut tmp_vec = layer[k..].iter().copied().collect::<Vec<_>>();
            tmp_vec.extend(layer[..k].iter());
            *layer = tmp_vec;
        }

        for l in 0..(m.min(n) / 2) {
            let mut pos = Position(l, l);
            let mut direction = Direction::Right;
            for offset in 0..(perimeter - 8 * l) {
                match direction {
                    Direction::Right => {
                        if pos.1 == n - l - 1 {
                            direction = Direction::Down;
                        }
                    }
                    Direction::Down => {
                        if pos.0 == m - l - 1 {
                            direction = Direction::Left;
                        }
                    }
                    Direction::Left => {
                        if pos.1 == l {
                            direction = Direction::Up;
                        }
                    }
                    Direction::Up => {}
                }
                grid[pos.0][pos.1] = layers[l][offset];
                pos += direction;
            }
        }

        grid
    }
}

#[derive(Clone, Copy)]
struct Position(usize, usize);

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl std::ops::AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Right => {
                self.1 += 1;
            }
            Direction::Down => {
                self.0 += 1;
            }
            Direction::Left => {
                self.1 -= 1;
            }
            Direction::Up => {
                self.0 -= 1;
            }
        }
    }
}
