pub struct Solution;

use std::collections::{HashMap, VecDeque};

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

impl Solution {
    pub fn min_moves(classroom: Vec<String>, energy_capacity: i32) -> i32 {
        let classroom: Vec<Vec<u8>> = classroom
            .iter()
            .map(|row| row.as_bytes().to_vec())
            .collect();
        let m = classroom.len();
        let n = classroom[0].len();

        let mut start_position = Position(-1, -1);
        let mut litter_mask_map: HashMap<Position, u16> = HashMap::new();
        for (row_index, row) in classroom.iter().enumerate() {
            for (col_index, &cell) in row.iter().enumerate() {
                let position = Position(row_index as isize, col_index as isize);
                if cell == b'S' {
                    start_position = position;
                } else if cell == b'L' {
                    let mask = 1 << litter_mask_map.len();
                    litter_mask_map.insert(position, mask);
                }
            }
        }
        if litter_mask_map.is_empty() {
            return 0;
        }
        let target_litter_mask = (1 << litter_mask_map.len()) - 1;

        let mut best_energy: Vec<Vec<Vec<i32>>> =
            vec![vec![vec![0; 1 << litter_mask_map.len()]; n]; m];

        let mut bfs_queue: VecDeque<BfsState> = VecDeque::new();
        bfs_queue.push_back(BfsState {
            position: start_position,
            energy: energy_capacity,
            litter_mask: 0,
            steps: 0,
        });

        while let Some(bfs_state) = bfs_queue.pop_front() {
            for direction in DIRECTIONS {
                let next_position = bfs_state.position + direction;

                let is_out_of_bound = next_position.0 < 0
                    || next_position.1 < 0
                    || next_position.0 as usize >= m
                    || next_position.1 as usize >= n;
                if is_out_of_bound {
                    continue;
                }

                let cell = classroom[next_position.0 as usize][next_position.1 as usize];

                let is_obstacle = cell == b'X';
                if is_obstacle {
                    continue;
                }

                let next_litter_mask = if let Some(&mask) = litter_mask_map.get(&next_position) {
                    bfs_state.litter_mask | mask
                } else {
                    bfs_state.litter_mask
                };
                if next_litter_mask == target_litter_mask {
                    return bfs_state.steps + 1;
                }

                let is_reset = cell == b'R';
                let next_energy = if is_reset {
                    energy_capacity
                } else {
                    bfs_state.energy - 1
                };
                let best_energy_of_next_position = best_energy[next_position.0 as usize]
                    [next_position.1 as usize][next_litter_mask as usize];
                if next_energy <= 0 || next_energy <= best_energy_of_next_position {
                    continue;
                }

                best_energy[next_position.0 as usize][next_position.1 as usize]
                    [next_litter_mask as usize] = next_energy;
                bfs_queue.push_back(BfsState {
                    position: next_position,
                    energy: next_energy,
                    litter_mask: next_litter_mask,
                    steps: bfs_state.steps + 1,
                });
            }
        }

        -1
    }
}

struct BfsState {
    position: Position,
    energy: i32,
    litter_mask: u16,
    steps: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position(isize, isize);

impl std::ops::Add<(isize, isize)> for Position {
    type Output = Self;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::min_moves(vec!["S.".to_string(), "XL".to_string()], 2),
        2
    );
    assert_eq!(Solution::min_moves(vec!["SL".to_string()], 1), 1);
}
