pub struct Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut result = 0i32;
        let mut last_point = &points[0];
        for point in points.iter().skip(1) {
            let diff_x = last_point[0].abs_diff(point[0]);
            let diff_y = last_point[1].abs_diff(point[1]);
            result += (diff_x.min(diff_y) + diff_x.abs_diff(diff_y)) as i32;
            last_point = point;
        }
        result
    }
}
