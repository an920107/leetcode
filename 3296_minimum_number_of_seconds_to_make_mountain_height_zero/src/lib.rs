pub struct Solution;

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let mut left = 0i64;
        let mut right = 10_000_000_000_000_000i64;
        let mut result = left;

        while left < right {
            let mid = (left + right) / 2;
            if worker_times
                .iter()
                .map(|&t| {
                    let t = t as i64;
                    let target = (mid * 2) / t;
                    if target == 0 {
                        return 0;
                    }
                    let mut k = (((1.0 + (4 * target) as f64).sqrt() - 1.0) / 2.0).floor() as i64;
                    while (k + 1) * (k + 2) <= target {
                        k += 1;
                    }
                    while k * (k + 1) > target {
                        k -= 1;
                    }
                    k
                })
                .sum::<i64>()
                >= mountain_height as i64
            {
                right = mid;
            } else {
                left = mid + 1;
                result = left;
            }
        }

        result
    }
}
