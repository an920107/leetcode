use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        let set: HashSet<i32> = HashSet::from_iter(friends.into_iter());
        let mut result = vec![];
        for num in order {
            if set.contains(&num) {
                result.push(num);
            }
        }
        result
    }
}
