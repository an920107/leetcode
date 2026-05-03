pub struct Solution;

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        let distance_x = (x - z).abs();
        let distance_y = (y - z).abs();
        if distance_x == distance_y {
            0
        } else {
            if distance_x < distance_y { 1 } else { 2 }
        }
    }
}
