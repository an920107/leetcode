pub struct Solution;

impl Solution {
    pub fn can_alice_win(mut n: i32) -> bool {
        let mut removing_count = 10;
        let mut turn = 0;

        while removing_count <= n {
            n -= removing_count;
            removing_count -= 1;
            turn = (turn + 1) % 2;
        }

        turn == 1
    }
}
