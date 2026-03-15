pub struct Solution;

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut k = k as usize - 1;
        Solution::recursion(n as usize, &mut k, "").unwrap_or("".to_string())
    }

    fn recursion(n_left: usize, k_left: &mut usize, current: &str) -> Option<String> {
        match (n_left, *k_left) {
            (0, 0) => return Some(current.to_string()),
            (0, _) => {
                *k_left -= 1;
                return None;
            }
            _ => {}
        }

        ('a'..='c')
            .filter(|c| !current.ends_with(*c))
            .map(|c| format!("{}{}", current, c))
            .fold(None, |state, s| {
                state.or_else(|| Solution::recursion(n_left - 1, k_left, &s))
            })
    }
}
