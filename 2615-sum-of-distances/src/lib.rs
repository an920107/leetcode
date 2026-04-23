pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let mut value_index_map: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut value_prefix_map: HashMap<i32, Vec<i64>> = HashMap::new();

        for (index, &num) in nums.iter().enumerate() {
            value_index_map
                .entry(num)
                .and_modify(|v| v.push(index))
                .or_insert(vec![index]);
            value_prefix_map
                .entry(num)
                .and_modify(|v| v.push(*v.last().unwrap() + index as i64))
                .or_insert(vec![0, index as i64]);
        }

        let mut result = vec![0; nums.len()];

        for (index, num) in nums.into_iter().enumerate() {
            let indecies = value_index_map.get(&num).unwrap();
            let prefix = value_prefix_map.get(&num).unwrap();

            let n = indecies.len();
            let i = indecies.binary_search(&index).unwrap();

            let left_sum = prefix[i] - prefix[0];
            let right_sum = prefix[n] - prefix[i + 1];
            let left_len = i as i64;
            let right_len = (n - i - 1) as i64;

            result[index] = right_sum - left_sum + (left_len - right_len) * index as i64;
        }

        result
    }
}
