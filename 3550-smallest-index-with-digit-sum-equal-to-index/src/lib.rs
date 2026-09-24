pub struct Solution;

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .enumerate()
            .filter(|&(i, n)| Self::sum_of_digits(n) == i as i32)
            .next()
            .map(|(i, _)| i as i32)
            .unwrap_or(-1)
    }

    fn sum_of_digits(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }
        sum
    }
}
