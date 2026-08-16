pub struct Solution;

impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let mut remains = [0i32; 3];
        for stone in stones.into_iter() {
            remains[(stone % 3) as usize] += 1;
        }

        if remains[0] % 2 == 0 {
            remains[1] > 0 && remains[2] > 0
        } else {
            remains[1].abs_diff(remains[2]) as i32 >= 3
        }
    }
}
