pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();

        for (index, num) in nums.iter().enumerate() {
            map.entry(*num)
                .and_modify(|v| v.push(index))
                .or_insert(vec![index]);
        }

        let mut result = i32::MAX;

        for (num_index, num) in nums.iter().enumerate() {
            let reversed_num = Self::reverse(*num);
            if let Some(indices) = map.get(&reversed_num) {
                let index_index = indices.partition_point(|&x| x <= num_index);
                if index_index >= indices.len() {
                    continue;
                }
                let reversed_num_index = indices[index_index];
                result = result.min(num_index.abs_diff(reversed_num_index) as i32);
            }
        }

        if result == i32::MAX { -1 } else { result }
    }

    fn reverse(mut num: i32) -> i32 {
        let mut reversed_num = 0;
        while num > 0 {
            reversed_num = reversed_num * 10 + num % 10;
            num /= 10;
        }
        reversed_num
    }
}
