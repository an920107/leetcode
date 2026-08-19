pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut row_indices: Vec<usize> =
            reserved_seats.iter().map(|v| v[0] as usize - 1).collect();
        row_indices.sort();

        let mut reserved_seats_heap: BinaryHeap<(usize, usize)> = BinaryHeap::from_iter(
            reserved_seats
                .into_iter()
                .map(|v| (v[0] as usize - 1, v[1] as usize - 1)),
        );

        let mut result = 2 * (n as usize - row_indices.len()) as i32;

        for row_index in row_indices.into_iter().rev() {
            let mut row = 0;

            while let Some(reserved_seat) = reserved_seats_heap.peek().copied()
                && reserved_seat.0 == row_index
            {
                reserved_seats_heap.pop();
                row |= 1 << reserved_seat.1;
            }

            if row & 0b_011_1111_110 == 0 {
                result += 2;
            } else if row & 0b_000_1111_000 == 0 {
                result += 1;
            } else {
                if row & 0b_011_1100_000 == 0 {
                    result += 1;
                }
                if row & 0b_000_0011_110 == 0 {
                    result += 1;
                }
            }
        }

        result
    }
}
