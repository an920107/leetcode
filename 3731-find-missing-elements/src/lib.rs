pub struct Solution;

const NUM_RANGE: usize = 101;

impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut max_num = i32::MIN;
        let mut min_num = i32::MAX;
        for &num in nums.iter() {
            max_num = max_num.max(num);
            min_num = min_num.min(num)
        }
        let mut exists = [false; NUM_RANGE];
        for num in nums.into_iter() {
            exists[num as usize] = true;
        }
        let mut result = vec![];
        for num in (min_num + 1)..max_num {
            if !exists[num as usize] {
                result.push(num);
            }
        }
        result
    }
}
