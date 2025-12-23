/*
 * @lc app=leetcode id=2054 lang=rust
 *
 * [2054] Two Best Non-Overlapping Events
 */

pub struct Solution;

// @lc code=start
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let events: Vec<Event> = events.iter().map(|v| Event::new(&v)).collect();

        let mut events_with_start: Vec<EventWithStart> =
            events.iter().map(|e| EventWithStart::from(*e)).collect();
        events_with_start.sort();

        let mut events_with_end: Vec<EventWithEnd> =
            events.iter().map(|e| EventWithEnd::from(*e)).collect();
        events_with_end.sort();

        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut result = 0;

        for event in events_with_end.iter().rev() {
            while let Some(top) = events_with_start.last() {
                if top.start <= event.end {
                    break;
                }
                heap.push(top.value);
                events_with_start.pop();
            }

            if let Some(max_value) = heap.peek() {
                result = result.max(event.value + max_value)
            }
        }

        for event in events {
            result = result.max(event.value);
        }

        result
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct EventWithStart {
    start: i32,
    value: i32,
}

impl From<Event> for EventWithStart {
    fn from(value: Event) -> Self {
        Self {
            start: value.start,
            value: value.value,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct EventWithEnd {
    end: i32,
    value: i32,
}

impl From<Event> for EventWithEnd {
    fn from(value: Event) -> Self {
        Self {
            end: value.end,
            value: value.value,
        }
    }
}

#[derive(Clone, Copy)]
struct Event {
    start: i32,
    end: i32,
    value: i32,
}

impl Event {
    fn new(raw: &[i32]) -> Self {
        Self {
            start: raw[0],
            end: raw[1],
            value: raw[2],
        }
    }
}
// @lc code=end
