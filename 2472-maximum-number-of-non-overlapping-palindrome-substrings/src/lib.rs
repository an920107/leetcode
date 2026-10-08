pub struct Solution;

/**
(0..s.len())
    .find(|&i| {
        (i..=i + 1).any(|x| x >= k && s[x - k..=i].bytes().eq(s[x - k..=i].bytes().rev()))
    })
    .map_or(0, |i| {
        1 + Self::max_palindromes(s[i + 1..].into(), k as i32)
    });
*/

impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        let k = k as usize;
        let s_bytes = s.as_bytes();

        let mut valid_palindromes: Vec<Vec<usize>> = vec![vec![]; s.len()];
        let mut palindrome_cores: Vec<(usize, usize)> = (0..s.len())
            .map(|index| (index, index + 1))
            .chain(
                s.as_bytes()
                    .windows(2)
                    .enumerate()
                    .filter(|(_, window)| window[0] == window[1])
                    .map(|(index, _)| (index, index + 2)),
            )
            .collect();
        while let Some(core) = palindrome_cores.pop() {
            if core.1 - core.0 >= k {
                valid_palindromes[core.0].push(core.1);
            }
            if core.0 > 0 && core.1 < s.len() && s_bytes[core.0 - 1] == s_bytes[core.1] {
                palindrome_cores.push((core.0 - 1, core.1 + 1));
            }
        }
        valid_palindromes.iter_mut().for_each(|v| v.sort());

        let mut memo = vec![None; s.len()];
        Self::recursion(&mut memo, &valid_palindromes, 0)
    }

    fn recursion(memo: &mut [Option<i32>], palindromes: &[Vec<usize>], index: usize) -> i32 {
        if index >= palindromes.len() {
            return 0;
        }

        if let Some(result) = memo[index] {
            return result;
        }

        let mut result = Self::recursion(memo, palindromes, index + 1);
        for end_index in palindromes.get(index).unwrap_or(&vec![]) {
            result = result.max(1 + Self::recursion(memo, palindromes, *end_index));
        }

        memo[index] = Some(result);
        result
    }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::max_palindromes("abaccdbbd".to_string(), 3), 2);
    assert_eq!(Solution::max_palindromes("adbcda".to_string(), 2), 0);
}
