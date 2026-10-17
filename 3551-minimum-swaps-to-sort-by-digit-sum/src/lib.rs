pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_swaps(mut nums: Vec<i32>) -> i32 {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_by(|&a, &b| {
            Self::digits_sum(a)
                .cmp(&Self::digits_sum(b))
                .then(a.cmp(&b))
        });
        let sorted_nums_indices_map: HashMap<i32, usize> = sorted_nums
            .into_iter()
            .enumerate()
            .map(|(index, num)| (num, index))
            .collect();

        let mut result = 0;
        for index in 0..nums.len() {
            let mut num = nums[index];
            while let Some(&target_index) = sorted_nums_indices_map.get(&num) {
                if index != target_index {
                    nums.swap(index, target_index);
                    result += 1;
                    num = nums[index];
                } else {
                    break;
                }
            }
        }
        result
    }

    fn digits_sum(mut num: i32) -> i32 {
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        sum
    }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::min_swaps(vec![22, 14, 33, 7]), 0);
    assert_eq!(Solution::min_swaps(vec![18, 43, 34, 16]), 2);
    assert_eq!(
        Solution::min_swaps(vec![978678783, 989995084, 465932830, 37255967]),
        3
    );
}
