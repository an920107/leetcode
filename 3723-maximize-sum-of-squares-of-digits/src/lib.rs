pub struct Solution;

impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        if sum > num * 9 {
            return "".to_string();
        }

        let mut result = String::with_capacity(num as usize);
        result.push_str(&"9".repeat(sum as usize / 9));
        if result.len() == num as usize {
            return result;
        }
        result.push_str(&(sum % 9).to_string());
        result.push_str(&"0".repeat(num as usize - result.len()));
        result
    }
}
