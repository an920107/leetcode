pub struct Solution;

use std::collections::{BTreeSet, VecDeque};

impl Solution {
    pub fn min_operations(s: String, k: i32) -> i32 {
        let k = k as usize;
        let n = s.len();
        let mut result = -1;

        let mut unvisited_even: BTreeSet<usize> =
            BTreeSet::from_iter((0..=s.len()).filter(|&n| n % 2 == 0));
        let mut unvisited_odd: BTreeSet<usize> =
            BTreeSet::from_iter((0..=s.len()).filter(|&n| n % 2 == 1));

        // (count of 0, layer)
        let mut bfs_queue: VecDeque<(usize, i32)> = VecDeque::new();
        let count_of_zero = s.bytes().filter(|&c| c == b'0').count();
        if count_of_zero % 2 == 1 && k % 2 == 0 {
            return -1;
        }
        bfs_queue.push_back((count_of_zero, 0));

        while let Some((count_of_zero, layer)) = bfs_queue.pop_front() {
            if count_of_zero == 0 {
                result = layer;
                break;
            }

            let count_of_one = n - count_of_zero;

            let lower_zeros_to_flip = k - k.min(count_of_one);
            let lower_ones_to_flip = k - lower_zeros_to_flip;
            let upper_zeros_to_flip = k.min(count_of_zero);
            let upper_ones_to_flip = k - upper_zeros_to_flip;

            let upper_bound = count_of_zero + lower_ones_to_flip - lower_zeros_to_flip;
            let lower_bound = count_of_zero + upper_ones_to_flip - upper_zeros_to_flip;

            let unvisied_set = if lower_bound % 2 == 0 {
                &mut unvisited_even
            } else {
                &mut unvisited_odd
            };

            let unvisied_vec: Vec<usize> = unvisied_set
                .range(lower_bound..=upper_bound)
                .copied()
                .collect();
            for new_count_of_zero in unvisied_vec {
                bfs_queue.push_back((new_count_of_zero, layer + 1));
                unvisied_set.remove(&new_count_of_zero);
            }
        }

        result
    }
}
