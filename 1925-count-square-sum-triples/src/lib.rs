pub struct Solution;

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut result = 0;

        for a in 1..=n {
            for b in (a + 1)..=n {
                let target = a * a + b * b;
                if target > n * n {
                    continue;
                }
                if let Some(_) = Solution::find_target_square(target, n) {
                    result += 1;
                }
            }
        }

        result * 2
    }

    fn find_target_square(target: i32, n: i32) -> Option<i32> {
        let mut lower = 1;
        let mut upper = n;
        while lower < upper {
            let mid = (lower + upper) / 2;
            let mid_square = mid * mid;

            if mid_square == target {
                return Some(mid);
            }

            if mid_square < target {
                lower = mid + 1;
            } else {
                upper = mid;
            }
        }

        if lower * lower == target {
            Some(lower)
        } else {
            None
        }
    }
}
