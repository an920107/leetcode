pub struct Solution;

use std::mem::swap;

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        let mut len1 = nums1.len();
        let mut len2 = nums2.len();

        let half = (len1 + len2 + 1) / 2;

        if len1 > len2 {
            swap(&mut len1, &mut len2);
            swap(&mut nums1, &mut nums2);
        }

        let mut left = 0;
        let mut right = len1;

        let mut left_max_1 = -f64::INFINITY;
        let mut right_min_1 = f64::INFINITY;
        let mut left_max_2 = -f64::INFINITY;
        let mut right_min_2 = f64::INFINITY;

        while left <= right {
            let mid = (left + right) / 2;
            let partition1 = mid;
            let partition2 = half - mid;

            left_max_1 = (partition1 > 0)
                .then(|| nums1[partition1 - 1] as f64)
                .unwrap_or(f64::NEG_INFINITY);

            right_min_1 = (partition1 < len1)
                .then(|| nums1[partition1] as f64)
                .unwrap_or(f64::INFINITY);

            left_max_2 = (partition2 > 0)
                .then(|| nums2[partition2 - 1] as f64)
                .unwrap_or(f64::NEG_INFINITY);

            right_min_2 = (partition2 < len2)
                .then(|| nums2[partition2] as f64)
                .unwrap_or(f64::INFINITY);

            if left_max_1 <= right_min_2 && left_max_2 <= right_min_1 {
                break;
            }

            if left_max_1 > right_min_2 {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        if (len1 + len2) % 2 == 1 {
            left_max_1.max(left_max_2)
        } else {
            (left_max_1.max(left_max_2) + right_min_1.min(right_min_2)) / 2.
        }
    }
}
