pub struct Solution;

use std::collections::BTreeSet;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        let x = x as usize;
        let k = k as usize;

        let mut costs: Vec<i64> = Vec::with_capacity(nums.len() - x + 1);
        let mut left: BTreeSet<(i32, usize)> = BTreeSet::new();
        let mut right: BTreeSet<(i32, usize)> = BTreeSet::new();
        let mut left_sum = 0i64;
        let mut right_sum = 0i64;
        for (index, &num) in nums[..x].iter().enumerate() {
            left.insert((num, index));
            left_sum += num as i64;
        }
        Self::balance(&mut left, &mut right, &mut left_sum, &mut right_sum);
        costs.push(Self::calculate_cost(&left, &right, left_sum, right_sum));
        for (index, window) in nums.windows(x + 1).enumerate() {
            let to_drop = (window[0], index);
            let to_add = (window[x], index + x);
            if left.remove(&to_drop) {
                left_sum -= to_drop.0 as i64;
            } else if right.remove(&to_drop) {
                right_sum -= to_drop.0 as i64;
            }
            Self::balance(&mut left, &mut right, &mut left_sum, &mut right_sum);
            if to_add.0 <= left.last().unwrap().0 {
                left.insert(to_add);
                left_sum += to_add.0 as i64;
            } else {
                right.insert(to_add);
                right_sum += to_add.0 as i64;
            }
            Self::balance(&mut left, &mut right, &mut left_sum, &mut right_sum);
            costs.push(Self::calculate_cost(&left, &right, left_sum, right_sum));
        }

        let mut memo: Vec<Vec<Option<Option<i64>>>> = vec![vec![None; k + 1]; costs.len()];
        Self::min_cost_from_index(&mut memo, &costs, 0, x, k).unwrap()
    }

    fn min_cost_from_index(
        memo: &mut Vec<Vec<Option<Option<i64>>>>,
        costs: &[i64],
        index: usize,
        x: usize,
        k: usize,
    ) -> Option<i64> {
        if k == 0 {
            return Some(0);
        }
        if index >= costs.len() {
            return None;
        }

        if let Some(result) = memo[index][k] {
            return result;
        }

        let skip = Self::min_cost_from_index(memo, costs, index + 1, x, k);
        let take =
            Self::min_cost_from_index(memo, costs, index + x, x, k - 1).map(|c| c + costs[index]);

        let result = match (skip, take) {
            (None, None) => None,
            (Some(v1), Some(v2)) => Some(v1.min(v2)),
            (Some(v), None) | (None, Some(v)) => Some(v),
        };
        memo[index][k] = Some(result);
        result
    }

    fn balance(
        left: &mut BTreeSet<(i32, usize)>,
        right: &mut BTreeSet<(i32, usize)>,
        left_sum: &mut i64,
        right_sum: &mut i64,
    ) {
        while right.len() > left.len() {
            let right_min = right.pop_first().unwrap();
            *right_sum -= right_min.0 as i64;
            *left_sum += right_min.0 as i64;
            left.insert(right_min);
        }
        while left.len() > right.len() + 1 {
            let left_max = left.pop_last().unwrap();
            *left_sum -= left_max.0 as i64;
            *right_sum += left_max.0 as i64;
            right.insert(left_max);
        }
    }

    fn calculate_cost(
        left: &BTreeSet<(i32, usize)>,
        right: &BTreeSet<(i32, usize)>,
        left_sum: i64,
        right_sum: i64,
    ) -> i64 {
        let median_offset = if left.len() > right.len() {
            left.last().unwrap().0 as i64
        } else {
            0
        };
        right_sum - left_sum + median_offset
    }
}
