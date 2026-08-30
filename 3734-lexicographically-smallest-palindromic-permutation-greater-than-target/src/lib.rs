pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn lex_palindromic_permutation(s: String, target: String) -> String {
        let mut s_char_counts = [0; 26];
        for c in s.bytes() {
            s_char_counts[(c - b'a') as usize] += 1;
        }

        let mut s_char_counts_odd_iter = s_char_counts
            .iter()
            .enumerate()
            .filter(|(_, count)| **count % 2 == 1);
        let middle_char = s_char_counts_odd_iter.next().map(|(c, _)| c);
        if s_char_counts_odd_iter.next().is_some() {
            return String::new();
        }

        s_char_counts.iter_mut().for_each(|count| *count /= 2);

        let mut half_result = vec![];
        return if Self::recursion(
            &mut s_char_counts,
            &target
                .bytes()
                .map(|c| (c - b'a') as usize)
                .collect::<Vec<_>>(),
            0,
            &mut half_result,
            middle_char,
            true,
        ) {
            half_result
                .iter()
                .copied()
                .chain(middle_char)
                .chain(half_result.iter().rev().copied())
                .map(|c| (c as u8 + b'a') as char)
                .collect()
        } else {
            String::new()
        };
    }

    fn recursion(
        s_char_counts: &mut [i32; 26],
        target: &[usize],
        index: usize,
        half_result: &mut Vec<usize>,
        middle_char: Option<usize>,
        same_flag: bool,
    ) -> bool {
        if index >= target.len() / 2 {
            return half_result
                .iter()
                .copied()
                .chain(middle_char)
                .chain(half_result.iter().rev().copied())
                .cmp(target.iter().copied())
                == Ordering::Greater;
        }

        if same_flag {
            if s_char_counts[target[index]] > 0 {
                s_char_counts[target[index]] -= 1;
                half_result.push(target[index]);
                if Self::recursion(
                    s_char_counts,
                    target,
                    index + 1,
                    half_result,
                    middle_char,
                    true,
                ) {
                    return true;
                }
                half_result.pop();
                s_char_counts[target[index]] += 1;
            }
            for (s_c, _) in s_char_counts
                .clone()
                .iter()
                .enumerate()
                .filter(|(s_c, count)| **count > 0 && *s_c > target[index])
            {
                s_char_counts[s_c] -= 1;
                half_result.push(s_c);
                if Self::recursion(
                    s_char_counts,
                    target,
                    index + 1,
                    half_result,
                    middle_char,
                    false,
                ) {
                    return true;
                }
                half_result.pop();
                s_char_counts[s_c] += 1;
            }
        } else {
            for (s_c, _) in s_char_counts
                .clone()
                .iter()
                .enumerate()
                .filter(|(_, count)| **count > 0)
            {
                s_char_counts[s_c] -= 1;
                half_result.push(s_c);
                if Self::recursion(
                    s_char_counts,
                    target,
                    index + 1,
                    half_result,
                    middle_char,
                    false,
                ) {
                    return true;
                }
                half_result.pop();
                s_char_counts[s_c] += 1;
            }
        }

        false
    }
}
