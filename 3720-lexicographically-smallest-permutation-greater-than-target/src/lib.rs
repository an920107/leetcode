pub struct Solution;

impl Solution {
    pub fn lex_greater_permutation(s: String, target: String) -> String {
        let mut s_char_count = [0; 26];
        for c in s.bytes() {
            s_char_count[(c - b'a') as usize] += 1;
        }

        let mut target_char_count = [0; 26];
        for c in target.bytes() {
            target_char_count[(c - b'a') as usize] += 1;
        }

        if let Some(result) = Self::recursion(
            s_char_count,
            target_char_count,
            &target
                .bytes()
                .map(|c| (c - b'a') as usize)
                .collect::<Vec<_>>(),
            0,
        ) {
            result
        } else {
            String::new()
        }
    }

    fn recursion(
        mut s_char_count: [i32; 26],
        mut target_char_count: [i32; 26],
        target: &[usize],
        index: usize,
    ) -> Option<String> {
        if index >= target.len() {
            return Some(String::new());
        }

        let target_char = target[index];
        target_char_count[target_char] -= 1;

        if s_char_count[target_char] > 0 {
            s_char_count[target_char] -= 1;
            if let Some(mut result) =
                Self::recursion(s_char_count, target_char_count, target, index + 1)
            {
                result = ((target_char as u8 + b'a') as char).to_string() + &result;
                if result
                    .bytes()
                    .map(|c| (c - b'a') as usize)
                    .collect::<Vec<_>>()
                    > target[index..].to_vec()
                {
                    return Some(result);
                }
            }
            s_char_count[target_char] += 1;
        }

        if let Some(c) = s_char_count
            .iter()
            .enumerate()
            .filter(|(c, count)| *c > target_char && **count > 0)
            .map(|(c, _)| c)
            .next()
        {
            s_char_count[c] -= 1;
            let mut smallest_s = String::with_capacity(target.len());
            smallest_s.push((c as u8 + b'a') as char);
            for (c, &count) in s_char_count.iter().enumerate() {
                smallest_s.push_str(
                    &((c as u8 + b'a') as char)
                        .to_string()
                        .repeat(count as usize),
                );
            }
            return Some(smallest_s);
        }

        None
    }
}

#[test]
fn test_solution() {
    Solution::lex_greater_permutation("ab".to_string(), "ab".to_string());
}
