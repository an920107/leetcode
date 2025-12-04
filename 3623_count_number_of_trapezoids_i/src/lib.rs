use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let mut y_count: HashMap<i32, i32> = HashMap::new();
        for point in points.iter() {
            let y = point[1];
            if let Some(count) = y_count.get_mut(&y) {
                *count += 1;
            } else {
                y_count.insert(y, 1);
            }
        }

        let combs: Vec<i64> = y_count
            .values()
            .map(|&n| n as i64 * (n - 1) as i64 / 2)
            .collect();

        let sum_square = combs.iter().sum::<i64>().pow(2);
        let square_sum = combs.iter().map(|c| c * c).sum::<i64>();

        ((sum_square - square_sum) / 2 % 1_000_000_007) as i32
    }
}
