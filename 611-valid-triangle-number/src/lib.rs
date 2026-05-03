pub struct Solution;

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        let mut count = 0;

        nums.sort();
        for (c_index, &c) in nums.iter().enumerate().skip(2) {
            let mut a_index = 0;
            let mut b_index = c_index - 1;

            while a_index < b_index {
                let a = nums[a_index];
                let b = nums[b_index];

                if a + b > c {
                    count += (b_index - a_index) as i32;
                    b_index -= 1;
                } else {
                    a_index += 1;
                }
            }
        }

        count
    }
}
