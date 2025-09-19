use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub struct NumberContainers {
    // k: idx, v: num
    numbers: HashMap<i32, i32>,

    // k: num, v: idxs
    min_heaps: HashMap<i32, BinaryHeap<Reverse<i32>>>,
}

impl NumberContainers {
    pub fn new() -> Self {
        Self {
            numbers: HashMap::new(),
            min_heaps: HashMap::new(),
        }
    }

    pub fn change(&mut self, index: i32, number: i32) {
        self.numbers.insert(index, number);
        if let Some(heap) = self.min_heaps.get_mut(&number) {
            heap.push(Reverse(index));
        } else {
            let mut heap = BinaryHeap::new();
            heap.push(Reverse(index));
            self.min_heaps.insert(number, heap);
        }
    }

    pub fn find(&mut self, number: i32) -> i32 {
        let mut result = -1;

        if let Some(heap) = self.min_heaps.get_mut(&number) {
            while let Some(Reverse(smallest_index)) = heap.peek() {
                if let Some(num_from_idx) = self.numbers.get(smallest_index)
                    && num_from_idx == &number
                {
                    result = *smallest_index;
                    break;
                } else {
                    heap.pop();
                }
            }
        }

        result
    }
}
