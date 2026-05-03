pub struct Solution;

use std::collections::BTreeSet;

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;

        // let lt_rb_prefix: Vec<Vec<i32>> = vec![vec![0]; m + n - 1];
        // let rt_lb_prefix: Vec<Vec<i32>> = vec![vec![0]; m + n - 1];

        // for i in (0..n).rev() {
        //     for offset in 0..m  {
        //         if i + offset >= n {
        //             break;
        //         }
        //         lt_rb_prefix[i]
        //     }
        // }

        let mut sums: BTreeSet<i32> = BTreeSet::new();

        for i in 0..m {
            for j in 0..n {
                sums.insert(grid[i as usize][j as usize]);

                let mut size = 1;
                while i - size >= 0 && j - size >= 0 && i + size < m && j + size < n {
                    let mut sum = 0;
                    let t = Point(i - size, j);
                    let r = Point(i, j + size);
                    let b = Point(i + size, j);
                    let l = Point(i, j - size);

                    let mut direction = Direction::T;
                    let mut current = t >> &direction;
                    sum += grid[current.0 as usize][current.1 as usize];

                    while current != t {
                        if current == r || current == b || current == l {
                            direction = direction.next();
                        }

                        current = current >> &direction;
                        sum += grid[current.0 as usize][current.1 as usize];
                    }

                    sums.insert(sum);
                    size += 1;
                }
            }
        }

        let mut result = vec![];
        while result.len() < 3
            && let Some(&num) = sums.last()
        {
            result.push(num);
            sums.remove(&num);
        }
        result
    }
}

enum Direction {
    T,
    R,
    B,
    L,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point(i32, i32);

impl Point {
    fn mov(&self, direction: &Direction) -> Self {
        let Self(r, c) = self;
        match direction {
            Direction::T => Self(r + 1, c + 1),
            Direction::R => Self(r + 1, c - 1),
            Direction::B => Self(r - 1, c - 1),
            Direction::L => Self(r - 1, c + 1),
        }
    }
}

impl core::ops::Shr<&Direction> for Point {
    type Output = Self;

    fn shr(self, rhs: &Direction) -> Self::Output {
        self.mov(rhs)
    }
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::T => Direction::R,
            Direction::R => Direction::B,
            Direction::B => Direction::L,
            Direction::L => Direction::T,
        }
    }
}
