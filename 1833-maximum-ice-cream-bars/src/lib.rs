pub struct Solution;

impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, coins: i32) -> i32 {
        costs.sort_unstable();
        costs
            .iter()
            .scan(0, |acc, &cost| {
                *acc += cost;
                if *acc <= coins { Some(*acc) } else { None }
            })
            .count() as i32
    }
}
