pub struct Solution;

impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let n = words.len() as i32;

        let mut result = -1;

        for distance in 0..=(n / 2) {
            let left = (start_index - distance + n) % n;
            let right = (start_index + distance) % n;

            if words[left as usize] == target || words[right as usize] == target {
                result = distance;
                break;
            }
        }

        result
    }
}
