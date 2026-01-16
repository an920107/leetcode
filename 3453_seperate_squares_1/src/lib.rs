pub struct Solution;

const THRESHOLD: f64 = 1e-6;

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let squares: Vec<Square> = squares.iter().map(|v| Square::from_raw(v)).collect();
        let total_area = squares.iter().map(|s| s.area()).sum::<f64>();
        let half_total_area = total_area / 2.0;

        let mut upper = 2.01e9f64;
        let mut lower = 0.0f64;
        let mut mid = (lower + upper) / 2.0;

        while (lower - upper).abs() > THRESHOLD {
            mid = (lower + upper) / 2.0;
            let area_above_mid = squares.iter().map(|s| s.area_above(mid)).sum::<f64>();
            if area_above_mid - half_total_area > THRESHOLD {
                lower = mid + 1e-5;
            } else {
                upper = mid;
            }
        }

        mid
    }
}

struct Square {
    y: f64,
    lenght: f64,
}

impl Square {
    fn from_raw(v: &[i32]) -> Self {
        Self {
            y: v[1] as f64,
            lenght: v[2] as f64,
        }
    }

    fn area(&self) -> f64 {
        self.lenght * self.lenght
    }

    fn area_above(&self, y: f64) -> f64 {
        if y < self.y {
            self.area()
        } else if y > self.y + self.lenght {
            0.0
        } else {
            (self.y + self.lenght - y) * self.lenght
        }
    }
}
