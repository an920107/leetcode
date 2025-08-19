pub struct Solution;

impl Solution {
    pub fn transform_array(nums: Vec<i32>) -> Vec<i32> {
        let even_count = nums.iter().filter(|&num| num % 2 == 0).count();
        let mut result = vec![0; even_count];
        result.extend(vec![1; nums.len() - even_count].iter());
        result 
    }
}
