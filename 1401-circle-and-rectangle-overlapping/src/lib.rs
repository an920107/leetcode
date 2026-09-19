pub struct Solution;

impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let circle = Circle {
            r: radius,
            cx: x_center,
            cy: y_center,
        };
        let rectangle = Rectangle { x1, y1, x2, y2 };

        if rectangle
            .get_points()
            .iter()
            .any(|point| circle.is_point_in(point.0, point.1))
        {
            return true;
        }

        if rectangle.is_point_in(circle.cx, circle.cy) {
            return true;
        }

        if circle
            .intersections_with_x_line(rectangle.x1)
            .iter()
            .any(|&y| y >= rectangle.y1 as f64 && y <= rectangle.y2 as f64)
        {
            return true;
        }
        if circle
            .intersections_with_x_line(rectangle.x2)
            .iter()
            .any(|&y| y >= rectangle.y1 as f64 && y <= rectangle.y2 as f64)
        {
            return true;
        }
        if circle
            .intersections_with_y_line(rectangle.y1)
            .iter()
            .any(|&x| x >= rectangle.x1 as f64 && x <= rectangle.x2 as f64)
        {
            return true;
        }
        if circle
            .intersections_with_y_line(rectangle.y2)
            .iter()
            .any(|&x| x >= rectangle.x1 as f64 && x <= rectangle.x2 as f64)
        {
            return true;
        }

        false
    }
}

struct Circle {
    r: i32,
    cx: i32,
    cy: i32,
}

impl Circle {
    fn intersections_with_x_line(&self, x: i32) -> Vec<f64> {
        let q = self.r * self.r - (x - self.cx) * (x - self.cx);
        if q < 0 {
            vec![]
        } else if q == 0 {
            vec![self.cy as f64]
        } else {
            vec![
                self.cy as f64 + (q as f64).sqrt(),
                self.cy as f64 - (q as f64).sqrt(),
            ]
        }
    }

    fn intersections_with_y_line(&self, y: i32) -> Vec<f64> {
        let q = self.r * self.r - (y - self.cy) * (y - self.cy);
        if q < 0 {
            vec![]
        } else if q == 0 {
            vec![self.cx as f64]
        } else {
            vec![
                self.cx as f64 + (q as f64).sqrt(),
                self.cx as f64 - (q as f64).sqrt(),
            ]
        }
    }

    fn is_point_in(&self, x: i32, y: i32) -> bool {
        self.r * self.r >= (self.cx - x) * (self.cx - x) + (self.cy - y) * (self.cy - y)
    }
}

struct Rectangle {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rectangle {
    fn get_points(&self) -> Vec<(i32, i32)> {
        vec![
            (self.x1, self.y1),
            (self.x1, self.y2),
            (self.x2, self.y2),
            (self.x2, self.y1),
        ]
    }

    fn is_point_in(&self, x: i32, y: i32) -> bool {
        x >= self.x1 && x <= self.x2 && y >= self.y1 && y <= self.y2
    }
}
