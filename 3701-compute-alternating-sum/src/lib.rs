pub struct Solution;

impl Solution {
    pub fn alternating_sum(nums: Vec<i32>) -> i32 {
        let (a, b): (Vec<_>, Vec<_>) = nums
            .into_iter()
            .enumerate()
            .partition(|(index, _)| index % 2 == 0);
        a.into_iter().map(|(_, n)| n).sum::<i32>() - b.into_iter().map(|(_, n)| n).sum::<i32>()
    }
}
