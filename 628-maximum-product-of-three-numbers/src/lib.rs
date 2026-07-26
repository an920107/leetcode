pub struct Solution;

impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort();
        nums[(n - 3)..]
            .iter()
            .product::<i32>()
            .max(nums[..2].iter().product::<i32>() * nums.last().unwrap())
    }
}
