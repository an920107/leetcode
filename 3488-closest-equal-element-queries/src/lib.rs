pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut value_map: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut index_map: Vec<usize> = vec![0; nums.len()];

        for (index, num) in nums.iter().enumerate() {
            value_map
                .entry(*num)
                .and_modify(|v| v.push(index))
                .or_insert(vec![index]);
            index_map[index] = value_map[num].len() - 1;
        }

        let mut result = vec![-1; queries.len()];

        for (query_index, query) in queries.into_iter().enumerate() {
            let num = match nums.get(query as usize) {
                Some(n) => *n,
                None => continue,
            };

            let indices = value_map.get(&num).unwrap();
            if indices.len() <= 1 {
                continue;
            }

            let index_index = index_map[query as usize];
            let index = indices[index_index];
            let left = indices[(index_index + 1) % indices.len()];
            let right = indices[(index_index + indices.len() - 1) % indices.len()];

            result[query_index] = (left.abs_diff(index))
                .min(left.min(index) + nums.len() - left.max(index))
                .min(right.abs_diff(index))
                .min(right.min(index) + nums.len() - right.max(index))
                as i32;
        }

        result
    }
}
