pub struct Solution;

impl Solution {
    pub fn find_final_value(mut nums: Vec<i32>, mut original: i32) -> i32 {
        nums.sort();

        while original <= 1000 {
            let result = nums.binary_search(&original);
            if result.is_err() {
                break;
            }
            original *= 2;
        }

        original
    }
}
