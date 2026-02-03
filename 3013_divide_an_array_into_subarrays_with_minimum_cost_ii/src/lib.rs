pub struct Solution;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let dist = dist as usize;

        let mut current_sum = nums[0] as i64;
        let mut min_heap: BinaryHeap<Reverse<i32>> =
            BinaryHeap::from_iter(nums[1..=(1 + dist)].iter().map(|&num| Reverse(num)));
        let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
        for _ in 0..(k - 1) {
            if let Some(Reverse(num)) = min_heap.pop() {
                max_heap.push(num);
                current_sum += num as i64;
            }
        }

        let mut result = current_sum;
        let mut min_heap_disposed_nums: HashMap<i32, usize> = HashMap::new();
        let mut max_heap_disposed_nums: HashMap<i32, usize> = HashMap::new();
        let mut max_heap_disposed_nums_count = 0;

        for i in 1..(n - dist - 1) {
            let to_remove = nums[i];
            let to_insert = nums[i + dist + 1];

            if to_remove > *max_heap.peek().unwrap_or(&0) {
                min_heap_disposed_nums
                    .entry(to_remove)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            } else {
                max_heap_disposed_nums
                    .entry(to_remove)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                max_heap_disposed_nums_count += 1;
                current_sum -= to_remove as i64;
            }

            if to_insert > *max_heap.peek().unwrap_or(&0) {
                min_heap.push(Reverse(to_insert));
            } else {
                max_heap.push(to_insert);
                current_sum += to_insert as i64;

                loop {
                    let popped = max_heap.pop().unwrap();
                    if let Some(count) = max_heap_disposed_nums.get_mut(&popped)
                        && *count > 0
                    {
                        *count -= 1;
                        max_heap_disposed_nums_count -= 1;
                    } else {
                        min_heap.push(Reverse(popped));
                        current_sum -= popped as i64;
                        break;
                    }
                }
            }

            while max_heap.len() - max_heap_disposed_nums_count < k - 1 {
                if let Some(Reverse(num)) = min_heap.pop() {
                    if let Some(count) = min_heap_disposed_nums.get_mut(&num)
                        && *count > 0
                    {
                        *count -= 1;
                    } else {
                        max_heap.push(num);
                        current_sum += num as i64;
                    }
                }
            }

            result = result.min(current_sum);
        }

        result
    }
}
