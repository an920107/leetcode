pub struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut result: Option<i32> = None;

        if let Some(rotating_operation_required) =
            Self::count_rotating_operation_required(&nums, SortingDirection::Increasing)
        {
            result = Some(
                result
                    .unwrap_or(i32::MAX)
                    .min(rotating_operation_required as i32),
            );
        } else if let Some(rotating_operation_required) =
            Self::count_rotating_operation_required(&nums, SortingDirection::Decreasing)
        {
            result = Some(
                result
                    .unwrap_or(i32::MAX)
                    .min(rotating_operation_required as i32 + 1),
            );
        }

        nums.reverse();
        if let Some(rotating_operation_required) =
            Self::count_rotating_operation_required(&nums, SortingDirection::Increasing)
        {
            result = Some(
                result
                    .unwrap_or(i32::MAX)
                    .min(rotating_operation_required as i32 + 1),
            );
        } else if let Some(rotating_operation_required) =
            Self::count_rotating_operation_required(&nums, SortingDirection::Decreasing)
        {
            result = Some(
                result
                    .unwrap_or(i32::MAX)
                    .min(rotating_operation_required as i32 + 2),
            );
        }

        result.unwrap_or(-1)
    }

    fn count_rotating_operation_required<T>(arr: &[T], direction: SortingDirection) -> Option<usize>
    where
        T: Ord,
    {
        let mut changing_count = 0;
        let mut changing_index = 0;
        for (index, window) in arr.windows(2).enumerate() {
            if (direction == SortingDirection::Increasing && window[1] < window[0])
                || (direction == SortingDirection::Decreasing && window[1] > window[0])
            {
                changing_count += 1;
                changing_index = index;
                if changing_count >= 2 {
                    return None;
                }
            }
        }

        if changing_count == 0 {
            return Some(0);
        }

        if (direction == SortingDirection::Increasing && arr[arr.len() - 1] <= arr[0])
            || (direction == SortingDirection::Decreasing && arr[arr.len() - 1] >= arr[0])
        {
            return Some(changing_index + 1);
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SortingDirection {
    Increasing,
    Decreasing,
}

#[test]
fn test_solution() {
    assert_eq!(Solution::min_operations(vec![1, 0, 2]), 2);
}
