pub struct Solution;

impl Solution {
    pub fn make_largest_special(s: String) -> String {
        Solution::recursion(&s)
    }

    fn recursion(s: &str) -> String {
        if s.is_empty() {
            return "".to_string();
        }

        let mut chunks: Vec<String> = vec![];
        let mut current_s: String = String::new();
        let mut prefix = 0;

        for c in s.bytes() {
            current_s.push(c as char);

            if c == b'1' {
                prefix += 1;
            } else {
                prefix -= 1;

                if prefix == 0 {
                    chunks.push(current_s.clone());
                    current_s.clear();
                }
            }
        }

        let mut best_chunks: Vec<String> = vec![];
        for chunk in chunks.iter() {
            let best = Solution::recursion(&chunk[1..(chunk.len() - 1)]);
            best_chunks.push("1".to_string() + &best + "0");
        }
        best_chunks.sort();
        best_chunks.reverse();
        best_chunks.join("")
    }
}
