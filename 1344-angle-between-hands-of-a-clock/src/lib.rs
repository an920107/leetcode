pub struct Solution;

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let minute_degree = minutes as f64 * 6.;
        let hour_degree = (hour % 12) as f64 * 30. + minutes as f64 * 0.5;

        let mut diff = (minute_degree - hour_degree).abs();
        if diff > 180. {
            diff = 360. - diff;
        }

        diff
    }
}
