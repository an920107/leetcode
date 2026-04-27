pub struct Solution;

impl Solution {
    pub fn max_distance(side_len: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
        let side_len = side_len as i64;
        let mut points: Vec<i64> = points
            .into_iter()
            .map(|p| {
                if p[1] == 0 {
                    p[0] as i64
                } else if p[0] as i64 == side_len {
                    side_len + p[1] as i64
                } else if p[1] as i64 == side_len {
                    side_len * 2 + (side_len - p[0] as i64)
                } else {
                    side_len * 3 + (side_len - p[1] as i64)
                }
            })
            .collect();
        points.sort_unstable();

        let mut right = side_len;
        let mut left = 1;
        let mut result = 1;

        while left <= right {
            let mid = (left + right) / 2;

            if Self::check(side_len as i64, &points, k, mid as i64) {
                left = mid + 1;
                result = mid;
            } else {
                right = mid - 1;
            }
        }

        result as i32
    }

    fn check(side_len: i64, points: &[i64], k: i32, target: i64) -> bool {
        let perimeter = side_len * 4;
        'start_point_iter: for start_point in points.iter() {
            let start_point = *start_point as i64;
            let end_point = perimeter + start_point - target;
            let mut curr_point = Some(start_point);

            for _ in 0..(k - 1) {
                curr_point = Self::next_point(points, target, curr_point.unwrap());

                if curr_point.is_none() || curr_point.unwrap() > end_point {
                    continue 'start_point_iter;
                }
            }

            return true;
        }

        false
    }

    fn next_point(points: &[i64], target: i64, curr_point: i64) -> Option<i64> {
        let min_next_point = curr_point + target;

        let mut left = 0;
        let mut right = points.len();

        while left < right {
            let mid = (left + right) / 2;
            if (points[mid] as i64) == min_next_point {
                return Some(points[mid] as i64);
            } else if (points[mid] as i64) < min_next_point {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        points.get(left).map(|p| *p as i64)
    }
}

#[test]
fn test_solution() {
    assert_eq!(
        2,
        Solution::max_distance(
            4,
            vec![vec![4, 4], vec![3, 4], vec![2, 0], vec![4, 3], vec![4, 0]],
            4
        )
    )
}
