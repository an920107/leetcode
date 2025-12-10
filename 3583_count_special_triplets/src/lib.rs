pub struct Solution;

use std::collections::HashMap;

const MOD: usize = 1_000_000_007;

impl Solution {
    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for (index, &num) in nums.iter().enumerate() {
            map.entry(num)
                .and_modify(|v| v.push(index))
                .or_insert(vec![index]);
        }

        let mut result = 0;
        for (&num, indices) in &map {
            if num == 0 {
                let n = indices.len();
                result = (result + n * (n - 1) * (n - 2) / 6) % MOD;
            } else if let Some(target_indices) = map.get(&(num * 2)) {
                for &index in indices {
                    let p = target_indices.partition_point(|&target_index| target_index < index);
                    let left_count = p;
                    let right_count = target_indices.len() - p;
                    result = (result + left_count * right_count) % MOD;
                }
            }
        }

        result as i32
    }
}
