pub struct Solution;

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let mut left = start;
        let mut right = start;
        let mut distance = 0;

        while left >= 0 || right < nums.len() as i32 {
            if nums.get(left as usize).copied() == Some(target)
                || nums.get(right as usize).copied() == Some(target)
            {
                break;
            }

            left -= 1;
            right += 1;

            distance += 1;
        }

        distance
    }
}
