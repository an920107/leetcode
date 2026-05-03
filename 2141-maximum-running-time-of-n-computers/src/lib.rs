pub struct Solution;

impl Solution {
    pub fn max_run_time(n: i32, mut batteries: Vec<i32>) -> i64 {
        batteries.sort();

        let mut left = 0;
        let mut right = batteries.iter().map(|&val| val as i64).sum::<i64>() / n as i64 + 1;

        while left < right {
            let mid = (left + right) / 2;
            if Solution::is_valid(mid, n, &batteries) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left - 1
    }

    fn is_valid(t: i64, n: i32, batteries: &[i32]) -> bool {
        t * n as i64 <= batteries.iter().map(|&power| (power as i64).min(t)).sum()
    }
}
