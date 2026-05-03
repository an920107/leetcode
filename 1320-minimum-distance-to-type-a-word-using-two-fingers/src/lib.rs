pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        let pos: Vec<(usize, usize)> = word.bytes().map(|c| Self::get_pos(c)).collect();
        let mut memo = HashMap::new();
        Self::recursion(
            &pos,
            &mut memo,
            State {
                index: 0,
                finger_1_last_index: None,
                finger_2_last_index: None,
            },
        )
    }

    fn recursion(pos: &Vec<(usize, usize)>, memo: &mut HashMap<State, i32>, state: State) -> i32 {
        if let Some(result) = memo.get(&state) {
            return *result;
        }

        if state.index >= pos.len() {
            return 0;
        }

        let with_finger_1 = Self::count_distance(
            pos[state.index],
            state
                .finger_1_last_index
                .map_or(pos[state.index], |index| pos[index]),
        ) + Self::recursion(
            pos,
            memo,
            State {
                index: state.index + 1,
                finger_1_last_index: Some(state.index),
                ..state
            },
        );

        let with_finger_2 = Self::count_distance(
            pos[state.index],
            state
                .finger_2_last_index
                .map_or(pos[state.index], |index| pos[index]),
        ) + Self::recursion(
            pos,
            memo,
            State {
                index: state.index + 1,
                finger_2_last_index: Some(state.index),
                ..state
            },
        );

        let result = with_finger_1.min(with_finger_2);
        memo.insert(state, result);
        result
    }

    fn get_pos(c: u8) -> (usize, usize) {
        let i = (c - b'A') as usize;
        (i / 6, i % 6)
    }

    fn count_distance(p1: (usize, usize), p2: (usize, usize)) -> i32 {
        (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)) as i32
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    index: usize,
    finger_1_last_index: Option<usize>,
    finger_2_last_index: Option<usize>,
}
