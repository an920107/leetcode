pub struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows_used = [[false; 9]; 9];
        let mut cols_used = [[false; 9]; 9];
        let mut blocks_used = [[false; 9]; 9];

        let mut transformed_board: [[Option<usize>; 9]; 9] = [[None; 9]; 9];

        for r in 0..9 {
            for c in 0..9 {
                let cell = board[r][c];
                let num = if cell >= '1' && cell <= '9' {
                    Some((cell as u8 - b'1') as usize)
                } else {
                    None
                };
                transformed_board[r][c] = num;
                if let Some(num) = num {
                    rows_used[r][num] = true;
                    cols_used[c][num] = true;
                    blocks_used[Solution::rc_to_block((r, c))][num] = true;
                }
            }
        }

        Solution::recursion(
            (0, 0),
            &mut transformed_board,
            &mut rows_used,
            &mut cols_used,
            &mut blocks_used,
        );

        for r in 0..9 {
            for c in 0..9 {
                board[r][c] = transformed_board[r][c]
                    .map(|n| (n as u8 + b'1') as char)
                    .unwrap_or('.');
            }
        }
    }

    fn recursion(
        (r, c): (usize, usize),
        board: &mut [[Option<usize>; 9]; 9],
        rows_used: &mut [[bool; 9]; 9],
        cols_used: &mut [[bool; 9]; 9],
        blocks_used: &mut [[bool; 9]; 9],
    ) -> bool {
        if r == 9 {
            return true;
        }

        let next_rc = Self::next_rc((r, c));

        if board[r][c].is_some() {
            return Self::recursion(next_rc, board, rows_used, cols_used, blocks_used);
        }

        let block = Self::rc_to_block((r, c));
        for num in 0..9 {
            let is_used_in_row = rows_used[r][num];
            let is_used_in_col = cols_used[c][num];
            let is_used_in_block = blocks_used[block][num];

            if is_used_in_row || is_used_in_col || is_used_in_block {
                continue;
            }

            board[r][c] = Some(num);
            rows_used[r][num] = true;
            cols_used[c][num] = true;
            blocks_used[block][num] = true;
            if Self::recursion(next_rc, board, rows_used, cols_used, blocks_used) {
                return true;
            }
            board[r][c] = None;
            rows_used[r][num] = is_used_in_row;
            cols_used[c][num] = is_used_in_col;
            blocks_used[block][num] = is_used_in_block;
        }

        false
    }

    fn rc_to_block((r, c): (usize, usize)) -> usize {
        let r_offset = r / 3;
        let c_offset = c / 3;
        r_offset * 3 + c_offset
    }

    fn next_rc((r, mut c): (usize, usize)) -> (usize, usize) {
        c += 1;
        (r + c / 9, c % 9)
    }
}
