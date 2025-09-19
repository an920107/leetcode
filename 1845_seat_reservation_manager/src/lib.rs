use std::{cmp::Reverse, collections::BinaryHeap};

pub struct SeatManager {
    unreserved_seats: BinaryHeap<Reverse<i32>>,
}

impl SeatManager {
    pub fn new(n: i32) -> Self {
        Self {
            unreserved_seats: BinaryHeap::from_iter((0..n).map(|i| Reverse(i))),
        }
    }

    pub fn reserve(&mut self) -> i32 {
        if let Some(Reverse(smallest_available_seat)) = self.unreserved_seats.peek() {
            let result = *smallest_available_seat + 1;
            self.unreserved_seats.pop();
            result
        } else {
            -1
        }
    }

    pub fn unreserve(&mut self, seat: i32) {
        self.unreserved_seats.push(Reverse(seat - 1));
    }
}
