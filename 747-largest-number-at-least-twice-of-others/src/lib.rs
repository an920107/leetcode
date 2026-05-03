pub struct Solution;

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut max_index = 0;
        let mut max_num = -1;
        for (index, &num) in nums.iter().enumerate() {
            if num > max_num {
                max_num = num;
                max_index = index;
            }
        }

        for num in nums {
            if num != max_num && max_num < num * 2 {
                return -1;
            }
        }

        max_index as i32
    }
}
