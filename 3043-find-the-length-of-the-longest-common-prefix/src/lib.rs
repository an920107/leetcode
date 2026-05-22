pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let prefix_set1 = Self::build_prefix_set(&arr1);
        let prefix_set2 = Self::build_prefix_set(&arr2);
        prefix_set1
            .intersection(&prefix_set2)
            .map(|s| s.len())
            .max()
            .unwrap_or(0) as i32
    }

    fn build_prefix_set(arr: &[i32]) -> HashSet<String> {
        let mut set = HashSet::new();
        for &num in arr.into_iter() {
            let num_string = num.to_string();
            let mut prefix = String::new();
            for c in num_string.chars() {
                prefix.push(c);
                set.insert(prefix.clone());
            }
        }
        set
    }
}
