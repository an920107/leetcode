pub struct Solution;

impl Solution {
    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        (num1..=num2).map(Self::count_waviness).sum::<i32>()
    }

    fn count_waviness(num: i32) -> i32 {
        let is_peak = |window: &[u8]| -> bool { window[1] > window[0] && window[1] > window[2] };
        let is_valley = |window: &[u8]| -> bool { window[1] < window[0] && window[1] < window[2] };

        num.to_string()
            .as_bytes()
            .windows(3)
            .filter(|&window| is_peak(window) || is_valley(window))
            .count() as i32
    }
}
