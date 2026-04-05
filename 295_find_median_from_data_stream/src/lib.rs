use std::{cmp::Reverse, collections::BinaryHeap};

pub struct MedianFinder {
    left_heap: BinaryHeap<i32>,
    right_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            left_heap: BinaryHeap::new(),
            right_heap: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        let median = self.find_median();

        if num as f64 >= median {
            self.right_heap.push(Reverse(num));
        } else {
            self.left_heap.push(num);
        }

        let left_len = self.left_heap.len();
        let right_len = self.right_heap.len();

        if left_len > right_len + 1 {
            let top = self.left_heap.pop().unwrap();
            self.right_heap.push(Reverse(top));
        }
        if right_len > left_len + 1 {
            let top = self.right_heap.pop().unwrap().0;
            self.left_heap.push(top);
        }
    }

    pub fn find_median(&self) -> f64 {
        let left_len = self.left_heap.len();
        let right_len = self.right_heap.len();

        if left_len == right_len {
            (*self.left_heap.peek().unwrap_or(&0) as f64
                + self.right_heap.peek().unwrap_or(&Reverse(0)).0 as f64)
                / 2.
        } else if left_len > right_len {
            *self.left_heap.peek().unwrap() as f64
        } else {
            self.right_heap.peek().unwrap().0 as f64
        }
    }
}
