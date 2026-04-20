pub struct Solution;

const COLOR_TYPES_COUNT: usize = 101;

impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let mut first_indecies: Vec<Option<usize>> = vec![None; COLOR_TYPES_COUNT];
        let mut result = 0;

        for (current_index, current_color) in colors.into_iter().enumerate() {
            let current_color = current_color as usize;

            if first_indecies[current_color].is_none() {
                first_indecies[current_color] = Some(current_index);
            }

            for other_color in 0..COLOR_TYPES_COUNT {
                if other_color == current_color {
                    continue;
                }

                if let Some(first_index) = first_indecies[other_color] {
                    result = result.max(current_index - first_index);
                }
            }
        }

        result as i32
    }
}
