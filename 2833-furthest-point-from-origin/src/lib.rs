pub struct Solution;

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let mut l_count = 0;
        let mut r_count = 0;
        let mut others_count = 0;

        for c in moves.bytes() {
            match c {
                b'L' => l_count += 1,
                b'R' => r_count += 1,
                b'_' => others_count += 1,
                _ => unreachable!(),
            };
        }

        if l_count >= r_count {
            l_count + others_count - r_count
        } else {
            r_count + others_count - l_count
        }
    }
}
