pub struct Solution;

impl Solution {
    pub fn jump(forwards: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = vec![0; forwards.len()];
        for (index, &forward) in forwards.iter().enumerate().rev().skip(1) {
            dp[index] = (1..=forward)
                .map(|offset| *dp.get(index + offset as usize).unwrap_or(&i32::MAX))
                .min()
                .unwrap_or(1_000_000_000)
                + 1;
        }
        dp[0]
    }
}
