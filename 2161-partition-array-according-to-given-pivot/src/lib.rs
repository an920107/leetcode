pub struct Solution;

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut smaller: Vec<i32> = Vec::with_capacity(nums.len());
        let mut equal: Vec<i32> = Vec::with_capacity(nums.len());
        let mut larger: Vec<i32> = Vec::with_capacity(nums.len());

        for num in nums.into_iter() {
            if num < pivot {
                smaller.push(num);
            } else if num > pivot {
                larger.push(num);
            } else {
                equal.push(num);
            }
        }

        let result = [smaller, equal, larger].concat();
        result
    }
}
