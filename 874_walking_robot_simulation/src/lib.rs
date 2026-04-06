pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let obstacles: HashSet<Point> =
            HashSet::from_iter(obstacles.iter().map(|v| Point { x: v[0], y: v[1] }));

        let mut direction = Direction::Up;
        let mut position = Point { x: 0, y: 0 };
        let mut max_distance = 0;

        for command in commands.into_iter() {
            match command {
                -2 => direction = direction.turned_left(),
                -1 => direction = direction.turned_right(),
                1..=9 => {
                    for _ in 0..command {
                        position += direction.into();
                        if obstacles.contains(&position) {
                            position -= direction.into();
                            break;
                        }
                        max_distance = max_distance.max(position.euclidean_distance())
                    }
                }
                _ => unreachable!(),
            }
        }

        max_distance
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turned_left(&self) -> Self {
        match self {
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
        }
    }

    fn turned_right(&self) -> Self {
        match self {
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl From<Direction> for Point {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Point { x: 0, y: 1 },
            Direction::Down => Point { x: 0, y: -1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }
}

impl Point {
    fn euclidean_distance(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution_1() {
        let real = Solution::robot_sim(vec![4, -1, 3], vec![]);
        let expected = 25;
        assert_eq!(real, expected);
    }

    #[test]
    fn test_solution_2() {
        let real = Solution::robot_sim(vec![6, -1, -1, 6], vec![vec![0, 0]]);
        let expected = 36;
        assert_eq!(real, expected);
    }

    #[test]
    fn test_solution_3() {
        let real = Solution::robot_sim(vec![7, -2, -2, 7, 5], vec![]);
        let expected = 49;
        assert_eq!(real, expected);
    }
}
