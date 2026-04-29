pub struct Solution;

impl Solution {
    pub fn contains_cycle(mut grid: Vec<Vec<char>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        grid = grid
            .into_iter()
            .map(|v| [vec!['\0'], v, vec!['\0']].concat())
            .collect();
        grid.insert(0, vec!['\0'; n + 2]);
        grid.push(vec!['\0'; n + 2]);

        let mut visited: Vec<Vec<bool>> = vec![vec![false; n + 2]; m + 2];

        for (i, row) in grid[..m + 1].iter().enumerate().skip(1) {
            for (j, cell) in row[..n + 1].iter().enumerate().skip(1) {
                if visited[i][j] {
                    continue;
                }

                let mut dfs_stack: Vec<DfsParams> = vec![DfsParams {
                    pos: Position(i, j),
                    from: None,
                }];

                while let Some(current) = dfs_stack.pop() {
                    if grid[current.pos.0][current.pos.1] != *cell {
                        continue;
                    }

                    if visited[current.pos.0][current.pos.1] {
                        return true;
                    }
                    visited[current.pos.0][current.pos.1] = true;

                    let directions = match current.from {
                        None => Direction::all(),
                        Some(direction) => direction.others(),
                    };

                    for direction in directions {
                        dfs_stack.push(DfsParams {
                            pos: current.pos + direction,
                            from: Some(-direction),
                        });
                    }
                }
            }
        }

        false
    }
}

#[derive(Debug, Clone, Copy)]
struct Position(usize, usize);

#[derive(Debug)]
struct DfsParams {
    pos: Position,
    from: Option<Direction>,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn all() -> Vec<Self> {
        vec![Self::Left, Self::Up, Self::Right, Self::Down]
    }

    fn others(&self) -> Vec<Self> {
        match self {
            Self::Left => {
                vec![Self::Up, Self::Right, Self::Down]
            }
            Self::Up => {
                vec![Self::Left, Self::Right, Self::Down]
            }
            Self::Right => {
                vec![Self::Left, Self::Up, Self::Down]
            }
            Self::Down => {
                vec![Self::Left, Self::Up, Self::Right]
            }
        }
    }
}

impl std::ops::Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Left => Position(self.0, self.1 - 1),
            Direction::Up => Position(self.0 - 1, self.1),
            Direction::Right => Position(self.0, self.1 + 1),
            Direction::Down => Position(self.0 + 1, self.1),
        }
    }
}

#[test]
fn test_solution() {
    let input = vec![
        vec!['a', 'a', 'a', 'a'],
        vec!['a', 'b', 'b', 'a'],
        vec!['a', 'b', 'b', 'a'],
        vec!['a', 'a', 'a', 'a'],
    ];
    assert_eq!(true, Solution::contains_cycle(input));

    let input = vec![
        vec!['a', 'b', 'b'],
        vec!['b', 'z', 'b'],
        vec!['b', 'b', 'a'],
    ];
    assert_eq!(false, Solution::contains_cycle(input));
}
