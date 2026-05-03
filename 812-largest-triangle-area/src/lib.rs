pub struct Solution;

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut result = 0.0_f64;

        for (a_index, a) in points.iter().enumerate() {
            for (b_index, b) in points.iter().enumerate().skip(a_index + 1) {
                for c in points.iter().skip(b_index + 1) {
                    let area = Solution::area((a[0], a[1]), (b[0], b[1]), (c[0], c[1]));
                    if area.is_nan() {
                        continue;
                    }
                    result = result.max(area);
                }
            }
        }

        result
    }

    fn distance(a: (i32, i32), b: (i32, i32)) -> f64 {
        ((a.0 - b.0).pow(2) as f64 + (a.1 - b.1).pow(2) as f64).sqrt()
    }

    fn area(a: (i32, i32), b: (i32, i32), c: (i32, i32)) -> f64 {
        let ab = Solution::distance(a, b);
        let bc = Solution::distance(b, c);
        let ac = Solution::distance(a, c);
        let s = (ab + bc + ac) / 2.0;

        (s * (s - ab) * (s - bc) * (s - ac)).sqrt()
    }
}
