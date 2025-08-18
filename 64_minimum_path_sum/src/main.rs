use std::cmp::Reverse;

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let directions: Vec<(usize, usize)> = vec![(1, 0), (0, 1)];
        let rows = grid.len();
        let cols = grid[0].len();

        let mut visited = vec![vec![false; cols]; rows];

        let mut distances = vec![vec![i32::MAX; cols]; rows];
        distances[0][0] = grid[0][0];

        let mut priority_queue = std::collections::BinaryHeap::<Reverse<Point>>::new();
        priority_queue.push(Reverse(Point {
            distance: distances[0][0],
            loc: (0, 0),
        }));

        while !priority_queue.is_empty() {
            let current = priority_queue.pop().unwrap().0;

            visited[current.loc.0][current.loc.1] = true;

            if current.loc == (rows - 1, cols - 1) {
                return current.distance;
            }

            for &direction in &directions {
                let new_loc = (current.loc.0 + direction.0, current.loc.1 + direction.1);
                if new_loc.0 >= rows || new_loc.1 >= cols {
                    continue;
                }
                if visited[new_loc.0][new_loc.1] {
                    continue;
                }

                let new_distance = current.distance + grid[new_loc.0][new_loc.1];
                if new_distance < distances[new_loc.0][new_loc.1] {
                    distances[new_loc.0][new_loc.1] = new_distance;
                    priority_queue.push(Reverse(Point {
                        distance: new_distance,
                        loc: new_loc,
                    }));
                }
            }
        }

        0
    }
}

#[derive(PartialEq, Eq, PartialOrd)]
struct Point {
    distance: i32,
    loc: (usize, usize),
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance
            .cmp(&other.distance)
            .then(self.loc.cmp(&other.loc))
    }
}
