pub struct Solution;

use std::{collections::BinaryHeap, ops::Range};

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, mut k: i32) -> i64 {
        let sparse_table = SparseTable::new(&nums);
        let mut max_heap: BinaryHeap<(i32, (usize, usize))> = BinaryHeap::new();

        let n = nums.len();
        for l in 0..n {
            max_heap.push((sparse_table.query(l..n), (l, n)));
        }

        let mut result = 0i64;
        while k > 0
            && let Some((value, (l, r))) = max_heap.pop()
        {
            result += value as i64;
            let new_r = r - 1;
            if new_r > l {
                max_heap.push((sparse_table.query(l..(new_r)), (l, new_r)));
            }
            k -= 1;
        }

        result
    }
}

struct SparseTable {
    min_table: Vec<Vec<i32>>,
    max_table: Vec<Vec<i32>>,
}

impl SparseTable {
    fn new(nums: &[i32]) -> Self {
        let n = nums.len();
        let max_k = nums.len().ilog2() as usize;

        let mut min_table = vec![vec![0; max_k + 1]; n];
        let mut max_table = vec![vec![0; max_k + 1]; n];

        for (i, &num) in nums.iter().enumerate() {
            min_table[i][0] = num;
            max_table[i][0] = num;
        }

        for k in 1..=max_k {
            for i in 0..=(n - (1 << k)) {
                min_table[i][k] = min_table[i][k - 1].min(min_table[i + (1 << (k - 1))][k - 1]);
                max_table[i][k] = max_table[i][k - 1].max(max_table[i + (1 << (k - 1))][k - 1]);
            }
        }

        Self {
            min_table,
            max_table,
        }
    }

    fn query(&self, range: Range<usize>) -> i32 {
        let len = range.len();
        let k = len.ilog2() as usize;
        let min = self.min_table[range.start][k].min(self.min_table[range.end - (1 << k)][k]);
        let max = self.max_table[range.start][k].max(self.max_table[range.end - (1 << k)][k]);
        max - min
    }
}
