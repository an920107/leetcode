pub struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut scores_stack: Vec<(usize, i32)> = Vec::with_capacity(s.len() / 2);
        let mut left_indices_stack: Vec<usize> = Vec::with_capacity(s.len() / 2);

        for (index, c) in s.bytes().enumerate() {
            match c {
                b'(' => {
                    left_indices_stack.push(index);
                }
                b')' => {
                    if let Some(left_index) = left_indices_stack.pop() {
                        let mut scores = 0;
                        loop {
                            if let Some((top_scores_index, top_scores)) = scores_stack.pop() {
                                if top_scores_index < left_index {
                                    scores_stack.push((top_scores_index, top_scores));
                                    break;
                                }
                                scores += top_scores;
                            } else {
                                break;
                            }
                        }
                        scores_stack.push((index, if scores == 0 { 1 } else { scores * 2 }));
                    }
                }
                _ => unreachable!(),
            }
        }

        scores_stack.into_iter().map(|(_, scores)| scores).sum()
    }
}
