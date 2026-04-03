pub struct Solution;

impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, mut walls: Vec<i32>) -> i32 {
        let mut robots: Vec<Robot> = robots
            .into_iter()
            .zip(distance.into_iter())
            .map(|(p, d)| Robot::new(p, d))
            .collect();
        let n = robots.len();

        robots.sort_by_key(|robot| robot.position);
        walls.sort();

        let mut walls_count_between_robots: Vec<i32> = vec![0; n];

        for i in 0..n {
            let robot = robots[i];

            let left_range = if i > 0 {
                (robot.position - robot.distance).max(robots[i - 1].position + 1)
            } else {
                robot.position - robot.distance
            };

            let right_range = if i < (n - 1) {
                (robot.position + robot.distance).min(robots[i + 1].position - 1)
            } else {
                robot.position + robot.distance
            };

            let left_wall_index = walls.partition_point(|&position| position < left_range);
            let mid_wall_index = walls.partition_point(|&position| position <= robot.position);
            let mid_wall_index_strict =
                walls.partition_point(|&position| position < robot.position);
            let right_wall_index = walls.partition_point(|&position| position <= right_range);

            robots[i].left_walls = (mid_wall_index - left_wall_index) as i32;
            robots[i].right_walls = (right_wall_index - mid_wall_index_strict) as i32;

            if i == 0 {
                continue;
            }

            let last_wall_index =
                walls.partition_point(|&position| position < robots[i - 1].position);
            walls_count_between_robots[i] = (mid_wall_index - last_wall_index) as i32;
        }

        let mut dp: Vec<(i32, i32)> = vec![(0, 0); n];
        dp[0] = (robots[0].left_walls, robots[0].right_walls);

        for i in 1..n {
            let left_robot = robots[i - 1];
            let robot = robots[i];

            dp[i].0 = (dp[i - 1].0 + robot.left_walls).max(
                if robot.distance + left_robot.distance >= robot.position - left_robot.position {
                    dp[i - 1].1 - left_robot.right_walls + walls_count_between_robots[i]
                } else {
                    dp[i - 1].1 + robot.left_walls
                },
            );
            dp[i].1 = dp[i - 1].0.max(dp[i - 1].1) + robot.right_walls;
        }

        dp[n - 1].0.max(dp[n - 1].1)
    }
}

#[derive(Clone, Copy)]
struct Robot {
    position: i32,
    distance: i32,
    left_walls: i32,
    right_walls: i32,
}

impl Robot {
    fn new(position: i32, distance: i32) -> Self {
        Self {
            position,
            distance,
            left_walls: 0,
            right_walls: 0,
        }
    }
}
