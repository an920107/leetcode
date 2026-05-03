pub struct Solution;

impl Solution {
    pub fn stone_game_iii(stones: Vec<i32>) -> String {
        let mut memo: Vec<Option<i32>> = vec![None; stones.len()];
        let diff = Self::max_scores(&stones, &mut memo, 0);

        if diff > 0 {
            "Alice".to_string()
        } else if diff < 0 {
            "Bob".to_string()
        } else {
            "Tie".to_string()
        }
    }

    fn max_scores(stones: &[i32], memo: &mut Vec<Option<i32>>, index: usize) -> i32 {
        if index >= stones.len() {
            return 0;
        }

        if let Some(result) = memo[index] {
            return result;
        }

        let mut result = i32::MIN;
        let mut current_sum = 0;
        for count in 0..3 {
            if index + count >= stones.len() {
                break;
            }
            current_sum += stones[index + count];
            result = result.max(current_sum - Self::max_scores(stones, memo, index + count + 1));
        }

        memo[index] = Some(result);
        result
    }
}
