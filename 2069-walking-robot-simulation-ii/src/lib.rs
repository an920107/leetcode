pub struct Robot {
    width: i32,
    height: i32,
    direction: Direction,
    position: Point,
    has_moved: bool,
}

impl Robot {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            direction: Direction::South,
            position: Point { x: 0, y: 0 },
            has_moved: false,
        }
    }

    pub fn step(&mut self, num: i32) {
        self.has_moved = true;

        for _ in 0..(num % self.perimeter()) {
            self.position += self.direction.into();

            if self.position.x < 0
                || self.position.y < 0
                || self.position.x >= self.width
                || self.position.y >= self.height
            {
                self.position -= self.direction.into();
                self.direction = self.direction.turned_left();
                self.position += self.direction.into();
            }
        }
    }

    pub fn get_pos(&self) -> Vec<i32> {
        self.position.into()
    }

    pub fn get_dir(&self) -> String {
        if self.has_moved {
            self.direction.to_string()
        } else {
            Direction::East.to_string()
        }
    }

    fn perimeter(&self) -> i32 {
        (self.width + self.height - 2) * 2
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::East => "East",
                Direction::North => "North",
                Direction::South => "South",
                Direction::West => "West",
            },
        )
    }
}

impl Direction {
    fn turned_left(&self) -> Self {
        match self {
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::West => Direction::South,
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
            Direction::North => Point { x: 0, y: 1 },
            Direction::South => Point { x: 0, y: -1 },
            Direction::West => Point { x: -1, y: 0 },
            Direction::East => Point { x: 1, y: 0 },
        }
    }
}

impl From<Point> for Vec<i32> {
    fn from(value: Point) -> Self {
        vec![value.x, value.y]
    }
}

#[test]
fn test_robot() {
    let mut robot = Robot::new(6, 3);
    robot.step(2);
    robot.step(2);
    assert_eq!(robot.get_pos(), vec![4, 0]);
    assert_eq!(robot.get_dir(), "East".to_string());
    robot.step(2);
    robot.step(1);
    robot.step(4);
    assert_eq!(robot.get_pos(), vec![1, 2]);
    assert_eq!(robot.get_dir(), "West".to_string());
}
