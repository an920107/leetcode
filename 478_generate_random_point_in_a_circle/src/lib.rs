pub struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
}

impl Solution {
    pub fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            radius,
            x_center,
            y_center,
        }
    }

    pub fn rand_point(&self) -> Vec<f64> {
        let mut x_rnd;
        let mut y_rnd;

        loop {
            x_rnd = Solution::rand() * 2.0 - 1.0;
            y_rnd = Solution::rand() * 2.0 - 1.0;

            if x_rnd * x_rnd + y_rnd * y_rnd <= 1.0 {
                break;
            }
        }

        vec![x_rnd * self.radius + self.x_center, y_rnd * self.radius + self.y_center]
    }

    fn rand() -> f64 {
        rand::random::<f64>()
    }
}
