pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut result = vec![];

        for k in 0..(1 << n) {
            let mut subset = vec![];
            let mut mask = 1 << n;
            for i in 0..n {
                mask >>= 1;
                if k & mask != 0 {
                    subset.push(nums[i]);
                }
            }
            result.push(subset);
        }

        result
    }
}
