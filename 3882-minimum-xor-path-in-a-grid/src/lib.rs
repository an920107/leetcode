pub struct Solution;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut xor_results: Vec<Vec<[bool; 1024]>> = vec![vec![[false; 1024]; n]; m];
        xor_results[0][0][grid[0][0] as usize] = true;

        for current_i in 0..m {
            for current_j in 0..n {
                if current_i == 0 && current_j == 0 {
                    continue;
                }

                let current_val = grid[current_i][current_j] as u16;
                let mut current_xor_result = [false; 1024];

                if current_i > 0 {
                    for last_xor_result in xor_results[current_i - 1][current_j]
                        .iter()
                        .enumerate()
                        .filter_map(|(num, b)| if *b { Some(num) } else { None })
                    {
                        current_xor_result[last_xor_result ^ current_val as usize] = true;
                    }
                }
                if current_j > 0 {
                    for last_xor_result in xor_results[current_i][current_j - 1]
                        .iter()
                        .enumerate()
                        .filter_map(|(num, b)| if *b { Some(num) } else { None })
                    {
                        current_xor_result[last_xor_result ^ current_val as usize] = true;
                    }
                }

                xor_results[current_i][current_j] = current_xor_result;
            }
        }

        xor_results[m - 1][n - 1]
            .into_iter()
            .position(|b| b)
            .unwrap() as i32
    }
}
