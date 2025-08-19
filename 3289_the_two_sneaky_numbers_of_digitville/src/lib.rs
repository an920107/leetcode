pub struct Solution;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut table = vec![false; nums.len() - 2];
        let mut result = vec![];

        for num in nums {
            if table[num as usize] {
                result.push(num);
            }
            table[num as usize] = true;
        }

        result
    }
}
