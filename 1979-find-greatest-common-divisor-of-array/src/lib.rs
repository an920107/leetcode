pub struct Solution;

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let max_num = nums.iter().max().copied().unwrap();
        let min_num = nums.iter().min().copied().unwrap();
        gcd(max_num as usize, min_num as usize) as i32
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 { b } else { gcd(b % a, a) }
}
