pub struct Solution;

impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let mut character_left_count = [0usize; 26];
        let mut seen = [false; 26];
        for c in s.bytes() {
            character_left_count[(c - b'a') as usize] += 1;
        }

        let mut stack: Vec<u8> = vec![];
        for c in s.bytes() {
            character_left_count[(c - b'a') as usize] -= 1;
            if seen[(c - b'a') as usize] {
                continue;
            }

            while let Some(&top) = stack.last()
                && top > c
                && character_left_count[(top - b'a') as usize] > 0
            {
                stack.pop();
                seen[(top - b'a') as usize] = false;
            }

            stack.push(c);
            seen[(c - b'a') as usize] = true;
        }

        stack.into_iter().map(|c| c as char).collect()
    }
}
