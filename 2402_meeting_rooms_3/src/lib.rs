/*
 * @lc app=leetcode id=2402 lang=rust
 *
 * [2402] Meeting Rooms III
 */

pub struct Solution;

// @lc code=start
use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut meetings: Vec<Meeting> = meetings
            .iter()
            .map(|v| Meeting {
                start: v[0] as usize,
                end: v[1] as usize,
            })
            .collect();

        meetings.sort();
        meetings.reverse();

        let mut available_rooms_indices: BinaryHeap<Reverse<usize>> =
            BinaryHeap::from_iter((0..n as usize).map(|i| Reverse(i)));
        let mut busy_rooms: BinaryHeap<Reverse<Room>> = BinaryHeap::new();
        let mut use_count: Vec<usize> = vec![0; n as usize];

        while let Some(meeting) = meetings.pop() {
            while let Some(&Reverse(room)) = busy_rooms.peek()
                && room.release_time <= meeting.start
            {
                busy_rooms.pop();
                available_rooms_indices.push(Reverse(room.index));
            }

            if let Some(Reverse(room_index)) = available_rooms_indices.pop() {
                busy_rooms.push(Reverse(Room {
                    release_time: meeting.end,
                    index: room_index,
                }));
                use_count[room_index] += 1;
            } else if let Some(Reverse(room)) = busy_rooms.pop() {
                busy_rooms.push(Reverse(Room {
                    release_time: meeting.end + (room.release_time - meeting.start),
                    index: room.index,
                }));
                use_count[room.index] += 1;
            }
        }

        use_count
            .into_iter()
            .enumerate()
            .max_by(|x, y| x.1.cmp(&y.1).then(y.0.cmp(&x.0)))
            .unwrap()
            .0 as i32
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Meeting {
    start: usize,
    end: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Room {
    release_time: usize,
    index: usize,
}
// @lc code=end
