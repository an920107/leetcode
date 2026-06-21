pub struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        gain.iter()
            .fold(
                SolutionState {
                    current: 0,
                    highest: 0,
                },
                |state, &x| state.update_with(x),
            )
            .highest
    }
}

struct SolutionState {
    current: i32,
    highest: i32,
}

impl SolutionState {
    fn update_with(&self, gain: i32) -> Self {
        Self {
            current: self.current + gain,
            highest: (self.current + gain).max(self.highest),
        }
    }
}
