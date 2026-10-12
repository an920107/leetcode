pub struct Solution;

impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let k = (k1 + k2) as usize;

        let mut diffs = nums1
            .iter()
            .zip(nums2.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .collect::<Vec<_>>();
        diffs.sort();

        let mut leftover = k;
        let mut left = 0;
        let mut right = diffs[diffs.len() - 1] + 1;
        while left < right {
            let mid = (left + right) / 2;
            if let Some(l) = Self::leftover(&diffs, mid, k) {
                leftover = l;
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        let base_diff = left;

        diffs
            .into_iter()
            .map(|diff| diff.min(base_diff) as i64)
            .map(|diff| diff * diff)
            .sum::<i64>()
            - (leftover as i64 * (2 * base_diff as i64 - 1)).max(0)
    }

    fn leftover(diffs: &[u32], base_diff: u32, k: usize) -> Option<usize> {
        let mut sum = 0u64;
        for diff in diffs {
            if *diff > base_diff {
                sum += (diff - base_diff) as u64;
            }
        }
        (k >= sum as usize).then(|| k - sum as usize)
    }
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::min_sum_square_diff(vec![10, 10, 10, 11, 5], vec![1, 0, 6, 6, 1], 11, 27),
        0
    );
}
