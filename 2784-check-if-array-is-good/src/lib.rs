pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        let n = nums.iter().max().copied().unwrap();
        let mut table: HashMap<i32, i32> = HashMap::from_iter((1..=n).map(|k| (k, 1)));
        table.insert(n, 2);

        for num in nums.into_iter() {
            *table.entry(num).or_default() -= 1;
        }

        table.iter().all(|(_, &v)| v == 0)
    }
}
