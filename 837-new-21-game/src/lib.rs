pub struct Solution;

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k == 0 || n >= k + max_pts {
            return 1f64;
        }

        let mut result = 0f64;
        let mut table = vec![0f64; (n + 1) as usize];
        table[0] = 1f64;
        let mut sum = 1f64;

        for i in 1..(n + 1) {
            let prob = sum / max_pts as f64;
            table[i as usize] = prob;

            if i < k {
                sum += prob;
            } else {
                result += prob;
            }

            if i >= max_pts {
                sum -= table[(i - max_pts) as usize];
            }
        }

        result
    }
}
