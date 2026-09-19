pub struct Solution;

impl Solution {
    pub fn min_sum_of_lengths(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums_prefix_sum = Vec::with_capacity(nums.len() + 1);
        nums_prefix_sum.push(0);
        for &num in nums.iter() {
            nums_prefix_sum.push(nums_prefix_sum.last().copied().unwrap() + num);
        }

        let mut intervals: Vec<(usize, usize)> = Vec::with_capacity(nums.len());
        for (current_i, &current_sum) in nums_prefix_sum.iter().enumerate() {
            if let Ok(offset) = nums_prefix_sum[(current_i + 1)..]
                .binary_search_by(|&next_sum| (next_sum - current_sum).cmp(&target))
            {
                intervals.push((current_i, current_i + offset + 1));
            }
        }

        let mut interval_lens_suffix_min = Vec::with_capacity(intervals.len() + 1);
        interval_lens_suffix_min.push(i32::MAX);
        for &(left, right) in intervals.iter().rev() {
            interval_lens_suffix_min.push(
                interval_lens_suffix_min
                    .last()
                    .copied()
                    .unwrap()
                    .min((right - left) as i32),
            );
        }
        interval_lens_suffix_min.reverse();

        let mut result: Option<i32> = None;
        for (current_index, &(current_left, current_right)) in intervals.iter().enumerate() {
            let current_len = (current_right - current_left) as i32;
            let next_index = intervals[(current_index + 1)..]
                .partition_point(|&(next_left, _)| next_left < current_right)
                + current_index
                + 1;
            if next_index == intervals.len() {
                continue;
            }
            let next_len = interval_lens_suffix_min[next_index];
            result = Some(result.unwrap_or(i32::MAX).min(current_len + next_len));
        }

        result.unwrap_or(-1)
    }
}
