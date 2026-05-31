pub struct Solution;

impl Solution {
    pub fn asteroids_destroyed(planet: i32, mut asteroids: Vec<i32>) -> bool {
        let mut planet = planet as i64;
        asteroids.sort_unstable();
        for asteroid in asteroids.into_iter() {
            if planet >= asteroid as i64 {
                planet += asteroid as i64
            } else {
                return false;
            }
        }
        true
    }
}
