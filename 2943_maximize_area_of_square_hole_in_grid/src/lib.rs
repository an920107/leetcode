pub struct Solution;

impl Solution {
    pub fn maximize_square_hole_area(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        let h_continuous_count = Solution::max_continuous_count(h_bars);
        let v_continuous_count = Solution::max_continuous_count(v_bars);
        let length = h_continuous_count.min(v_continuous_count) + 1;
        length * length
    }

    fn max_continuous_count(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums.push(i32::MAX);
        let mut result = 1;
        let mut count = 1;
        let mut last_num = nums[0];
        for num in nums.iter().skip(1) {
            if *num == last_num + 1 {
                count += 1;
            } else {
                result = result.max(count);
                count = 1;
            }
            last_num = *num;
        }
        result
    }
}
