pub struct Solution;

impl Solution {
    pub fn gcd_sum(nums: Vec<i32>) -> i64 {
        let mut prefix_gcd: Vec<i32> = nums
            .iter()
            .scan(0i32, |current_max, &num| {
                *current_max = num.max(*current_max);
                Some(Self::gcd(*current_max as usize, num as usize) as i32)
            })
            .collect();
        prefix_gcd.sort_unstable();
        prefix_gcd
            .iter()
            .zip(prefix_gcd.iter().rev())
            .take(prefix_gcd.len() / 2)
            .map(|(&a, &b)| Self::gcd(a as usize, b as usize) as i64)
            .sum()
    }

    fn gcd(a: usize, b: usize) -> usize {
        if a == 0 { b } else { Self::gcd(b % a, a) }
    }
}
