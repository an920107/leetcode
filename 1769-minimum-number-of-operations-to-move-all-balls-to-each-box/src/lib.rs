pub struct Solution;

impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let boxes: Vec<i32> = boxes.chars().map(|c| c as i32 - 48).collect();

        let mut balls_counted_from_left = vec![0; boxes.len()];
        for i in 1..boxes.len() {
            balls_counted_from_left[i] = balls_counted_from_left[i - 1] + boxes[i - 1];
        }

        let mut balls_counted_from_right = vec![0; boxes.len()];
        for i in (0..(boxes.len() - 1)).rev() {
            balls_counted_from_right[i] = balls_counted_from_right[i + 1] + boxes[i + 1];
        }

        let mut steps_moving_to_right = vec![0; boxes.len()];
        for i in 1..boxes.len() {
            steps_moving_to_right[i] = steps_moving_to_right[i - 1] + balls_counted_from_left[i];
        }

        let mut steps_moving_to_left = vec![0; boxes.len()];
        for i in (0..(boxes.len() - 1)).rev() {
            steps_moving_to_left[i] = steps_moving_to_left[i + 1] + balls_counted_from_right[i];
        }

        steps_moving_to_left
            .iter()
            .zip(steps_moving_to_right.iter())
            .map(|(&l, &r)| l + r)
            .collect()
    }
}
