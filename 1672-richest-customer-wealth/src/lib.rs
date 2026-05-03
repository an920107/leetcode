pub struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .iter()
            .map(|assets| assets.iter().sum())
            .max()
            .unwrap()
    }
}
