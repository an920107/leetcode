pub struct Solution;

struct DFSParams {
    occupied_loc: Vec<(usize, usize)>,
    available_status: Vec<Vec<bool>>,
}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let queen_directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (1, 1), (1, -1)];

        let mut result = 0;

        let mut stack: Vec<DFSParams> = vec![DFSParams {
            occupied_loc: vec![],
            available_status: vec![vec![true; n]; n],
        }];

        while let Some(top) = stack.pop() {
            if top.occupied_loc.len() == n {
                result += 1;
                continue;
            }

            if top
                .available_status
                .iter()
                .all(|row| row.iter().all(|available| !available))
            {
                continue;
            }

            let last_occupied_linear_index = top.occupied_loc.last().map(|loc| loc.0 * n + loc.1);

            for linear_index in (last_occupied_linear_index.map_or(0, |index| index + 1))..(n * n) {
                let loc = (linear_index / n, linear_index % n);
                if !top.available_status[loc.0][loc.1] {
                    continue;
                }

                let mut occupied_loc = top.occupied_loc.clone();
                let mut available_status = top.available_status.clone();

                occupied_loc.push(loc);

                for direction in queen_directions.iter() {
                    let mut current_loc = (loc.0 as i32, loc.1 as i32);
                    current_loc.0 += direction.0;
                    current_loc.1 += direction.1;
                    while current_loc.0 >= 0
                        && current_loc.1 >= 0
                        && (current_loc.0 as usize) < n
                        && (current_loc.1 as usize) < n
                    {
                        available_status[current_loc.0 as usize][current_loc.1 as usize] = false;
                        current_loc.0 += direction.0;
                        current_loc.1 += direction.1;
                    }
                }

                stack.push(DFSParams {
                    occupied_loc,
                    available_status,
                });
            }
        }

        result
    }
}
