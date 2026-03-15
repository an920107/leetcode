pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut front_index = 0i32;
        let mut back_index = (nums.len() - 1) as i32;

        for index in 0..nums.len() {
            while nums[index] != 1 {
                if (index as i32) < front_index || (index as i32) > back_index {
                    break;
                }
                if nums[index] == 0 {
                    nums.swap(index, front_index as usize);
                    front_index += 1;
                } else if nums[index] == 2 {
                    nums.swap(index, back_index as usize);
                    back_index -= 1;
                }
            }
        }
    }
}
