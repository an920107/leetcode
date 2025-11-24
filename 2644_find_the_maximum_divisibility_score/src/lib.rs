pub struct Solution;

impl Solution {
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        divisors
            .iter()
            .map(|d| (nums.iter().filter(|n| *n % *d == 0).count() as i32, *d))
            .max_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)))
            .unwrap()
            .1
    }
}
