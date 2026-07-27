pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .scan((i32::MIN, i32::MIN), |state, num| {
                if num > state.0 {
                    state.1 = state.0;
                    state.0 = num;
                } else if num > state.1 {
                    state.1 = num;
                }
                Some((state.0 - 1) * (state.1 - 1))
            })
            .last()
            .unwrap()
    }
}
