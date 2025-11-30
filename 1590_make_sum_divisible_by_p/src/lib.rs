use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let mut prefixs: Vec<i32> = vec![0];
        for &num in nums.iter() {
            prefixs.push((num + prefixs.last().unwrap()) % p);
        }

        let r = prefixs.last().unwrap() % p;
        if r == 0 {
            return 0;
        }

        let mut result = nums.len() as i32;
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (i, &num) in prefixs.iter().enumerate().rev() {
            if let Some(&j) = map.get(&((num + r) % p)) {
                result = result.min((j - i) as i32);
            }

            map.insert(num % p, i);
        }

        if result == nums.len() as i32 {
            -1
        } else {
            result
        }
    }
}
