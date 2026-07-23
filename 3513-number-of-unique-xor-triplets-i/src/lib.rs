pub struct Solution;

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        match nums.len() {
            1 => 1,
            2 => 2,
            3 => 4,
            k => 2i32.pow(k.ilog2() + 1),
        }
    }
}
