pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        for length in (2..=nums.len()).rev() {
            let mut num_count: HashMap<i32, i32> = HashMap::new();

            for index in 0..length {
                Solution::count_increase(&mut num_count, nums[index], 1);
            }
            if Solution::is_balanced(&num_count) {
                return length as i32;
            }

            for begin_index in 1..(nums.len() - length + 1) {
                let end_index = begin_index + length - 1;
                Solution::count_increase(&mut num_count, nums[begin_index - 1], -1);
                Solution::count_increase(&mut num_count, nums[end_index], 1);
                if Solution::is_balanced(&num_count) {
                    return length as i32;
                }
            }
        }

        0
    }

    fn count_increase(num_count: &mut HashMap<i32, i32>, num: i32, increment: i32) {
        num_count
            .entry(num)
            .and_modify(|count| *count += increment)
            .or_insert(1);

        if num_count.get(&num).unwrap() <= &0 {
            num_count.remove(&num);
        }
    }

    fn is_balanced(num_count: &HashMap<i32, i32>) -> bool {
        let (even, odd): (Vec<i32>, Vec<i32>) = num_count.keys().partition(|&n| n % 2 == 0);
        odd.len() == even.len()
    }
}
