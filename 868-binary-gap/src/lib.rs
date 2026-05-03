pub struct Solution;

impl Solution {
    pub fn binary_gap(mut n: i32) -> i32 {
        let mut current_distance: Option<i32> = None;
        let mut result = 0;

        while n > 0 {
            if n & 1 == 0 {
                current_distance = current_distance.and_then(|d| Some(d + 1));
            } else {
                result = result.max(current_distance.unwrap_or(0));
                current_distance = Some(1);
            }
            n >>= 1;
        }

        result
    }
}
