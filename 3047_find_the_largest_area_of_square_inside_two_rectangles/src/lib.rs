/*
 * @lc app=leetcode id=3047 lang=rust
 *
 * [3047] Find the Largest Area of Square Inside Two Rectangles
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let rectangles: Vec<Rectangle> = bottom_left
            .iter()
            .zip(top_right.iter())
            .map(|r| Rectangle::from_lb_rt((r.0[0], r.0[1]), (r.1[0], r.1[1])))
            .collect();

        let mut result = 0i64;
        for (i, rect_a) in rectangles.iter().enumerate() {
            for rect_b in rectangles[i + 1..rectangles.len()].iter() {
                let intersection_rect = rect_a.intersect(rect_b);
                if let Some(rect) = intersection_rect {
                    result = result.max(rect.max_square_area());
                }
            }
        }

        result
    }
}

pub struct Rectangle {
    lb: (i32, i32),
    rt: (i32, i32),
}

impl Rectangle {
    pub fn from_lb_rt(lb: (i32, i32), rt: (i32, i32)) -> Self {
        Self { lb, rt }
    }

    fn max_square_area(&self) -> i64 {
        let w = self.rt.0 - self.lb.0;
        let h = self.rt.1 - self.lb.1;
        let l = w.min(h) as i64;
        l * l
    }

    pub fn intersect(&self, other: &Rectangle) -> Option<Rectangle> {
        let lb = (self.lb.0.max(other.lb.0), self.lb.1.max(other.lb.1));
        let rt = (self.rt.0.min(other.rt.0), self.rt.1.min(other.rt.1));

        if lb.0 < rt.0 && lb.1 < rt.1 {
            Some(Rectangle::from_lb_rt(lb, rt))
        } else {
            None
        }
    }
}
// @lc code=end
