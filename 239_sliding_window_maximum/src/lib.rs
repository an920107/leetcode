pub struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut count: BTreeMap<i32, i32> = BTreeMap::new();

        for i in 0..k {
            let num = nums[i];
            count.entry(num).and_modify(|cnt| *cnt += 1).or_insert(1);
        }

        let mut result: Vec<i32> = vec![*count.last_entry().unwrap().key()];

        for i in 0..(nums.len() - k) {
            let num = nums[i];
            count.entry(num).and_modify(|cnt| *cnt -= 1);
            if count.get(&num) == Some(&0) {
                count.remove(&num);
            }

            let num = nums[i + k];
            count.entry(num).and_modify(|cnt| *cnt += 1).or_insert(1);

            result.push(*count.last_entry().unwrap().key());
        }

        result
    }
}
