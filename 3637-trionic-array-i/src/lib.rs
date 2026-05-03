pub struct Solution;

impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let mut direction = Direction::Up(0);

        for i in 1..nums.len() {
            if let Some(next_direction) = direction.check(nums[i - 1], nums[i]) {
                direction = next_direction;
            } else {
                return false;
            }
        }

        direction == Direction::UpThanDown(3)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up(i32),
    UpThanDown(i32),
    DownThanUp(i32),
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::Up(cnt) => Direction::UpThanDown(*cnt + 1),
            Direction::UpThanDown(cnt) => Direction::DownThanUp(*cnt + 1),
            Direction::DownThanUp(cnt) => Direction::UpThanDown(*cnt + 1),
        }
    }

    fn check(&self, a: i32, b: i32) -> Option<Direction> {
        match self {
            Direction::Up(_) => {
                if a < b {
                    Some(self.next())
                } else {
                    None
                }
            }
            Direction::UpThanDown(_) => {
                if a < b {
                    Some(*self)
                } else if a > b {
                    Some(self.next())
                } else {
                    None
                }
            }
            Direction::DownThanUp(_) => {
                if a > b {
                    Some(*self)
                } else if a < b {
                    Some(self.next())
                } else {
                    None
                }
            }
        }
    }
}
