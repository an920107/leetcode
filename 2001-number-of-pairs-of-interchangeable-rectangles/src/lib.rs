pub struct Solution;

impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        let mut groups = std::collections::HashMap::new();

        for rect in rectangles {
            let gcd = Solution::gcd(rect[0], rect[1]);
            let ratio = (rect[0] / gcd, rect[1] / gcd);
            groups
                .entry(ratio)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        groups.values().map(|&n| n * (n - 1) / 2).sum()
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 {
            b
        } else if a <= b {
            Solution::gcd(b % a, a)
        } else {
            Solution::gcd(b, a)
        }
    }
}
