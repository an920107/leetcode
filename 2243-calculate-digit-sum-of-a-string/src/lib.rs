pub struct Solution;

impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        Self::recursion(&s, k as usize)
    }

    fn recursion(s: &str, k: usize) -> String {
        if s.len() <= k {
            return s.to_owned();
        }

        let mut result = String::with_capacity(s.len() / k + 1);
        for chunk in s.as_bytes().chunks(k) {
            let sum = chunk.iter().map(|&c| (c - b'0') as i32).sum::<i32>();
            result.push_str(&sum.to_string());
        }

        Self::recursion(&result, k)
    }
}
