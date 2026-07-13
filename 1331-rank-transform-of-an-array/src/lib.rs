pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut ranks: Vec<i32> = vec![0; arr.len()];
        let mut last_value = i32::MIN;
        let mut current_rank = 0;

        let mut min_heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::from_iter(
            arr.into_iter()
                .enumerate()
                .map(|(index, value)| Reverse((value, index))),
        );

        while let Some(Reverse((value, index))) = min_heap.pop() {
            if value != last_value {
                current_rank += 1;
            }
            ranks[index] = current_rank;
            last_value = value;
        }

        ranks
    }
}
