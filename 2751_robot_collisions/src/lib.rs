pub struct Solution;

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let n = positions.len();
        let directions: Vec<Direction> = directions.bytes().map(Direction::from).collect();

        let mut robots: Vec<Robot> = vec![];
        for i in 0..n {
            robots.push(Robot {
                id: i,
                position: positions[i],
                health: healths[i],
                directrion: directions[i],
            });
        }

        robots.sort_by_key(|robot| robot.position);

        let mut stack: Vec<Robot> = vec![];

        'robots_for_loop: for robot in robots.iter() {
            let mut robot = *robot;

            while let Some(top) = stack.pop() {
                if !top.will_collide_with(&robot) {
                    stack.push(top);
                    stack.push(robot);
                    break;
                }

                if top.health == robot.health {
                    continue 'robots_for_loop;
                } else if top.health > robot.health {
                    stack.push(top.with_health_decreased());
                    break;
                } else {
                    robot = robot.with_health_decreased();
                }
            }

            if stack.is_empty() {
                stack.push(robot);
            }
        }

        stack.sort_by_key(|robot| robot.id);
        stack.iter().map(|robot| robot.health).collect()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    fn will_collide_with(&self, other: &Self) -> bool {
        self == &Direction::Right && other == &Direction::Left
    }
}

#[derive(Clone, Copy)]
struct Robot {
    id: usize,
    position: i32,
    health: i32,
    directrion: Direction,
}

impl Robot {
    fn will_collide_with(&self, other: &Self) -> bool {
        self.directrion.will_collide_with(&other.directrion)
    }

    fn with_health_decreased(&self) -> Self {
        Self {
            health: self.health - 1,
            ..*self
        }
    }
}
