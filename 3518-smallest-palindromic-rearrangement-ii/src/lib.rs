pub struct Solution;

use std::collections::HashMap;

const MAX_N: usize = 10_000;

impl Solution {
    pub fn smallest_palindrome(s: String, k: i32) -> String {
        let primes = Self::find_primes_less_than(MAX_N / 2);
        let mut k = k;

        let mut frequency = [0; 26];
        for c in s.bytes() {
            frequency[(c - b'a') as usize] += 1;
        }

        let mut log_fact = vec![0.0; MAX_N / 2 + 1];
        for l in 2..=MAX_N / 2 {
            log_fact[l] = log_fact[l - 1] + (l as f64).log10();
        }

        let mut mid_char: Option<u8> = None;
        for (index, count) in frequency.iter_mut().enumerate() {
            let c = index as u8 + b'a';
            if *count % 2 == 1 {
                mid_char = Some(c)
            }
            *count /= 2;
        }

        let mut left_part = String::new();
        while left_part.len() < s.len() / 2 {
            let mut last_c: Option<char> = None;
            for candidate in 0..26 {
                if frequency[candidate] > 0 {
                    frequency[candidate] -= 1;

                    let current_n = frequency.iter().sum::<i32>();

                    if let Some(new_k) = Self::is_less_and_take_subtract(
                        k, current_n, &frequency, &primes, &log_fact,
                    ) {
                        k = new_k;
                        frequency[candidate] += 1;
                    } else {
                        last_c = Some((candidate as u8 + b'a') as char);
                        frequency[candidate] += 1;
                        break;
                    }
                }
            }
            if let Some(c) = last_c {
                frequency[(c as u8 - b'a') as usize] -= 1;
                left_part.push(c);
            } else {
                return String::new();
            }
        }

        let right_part = left_part.chars().rev().collect::<String>();
        let mid_part = if let Some(c) = mid_char {
            (c as char).to_string()
        } else {
            String::new()
        };
        left_part + &mid_part + &right_part
    }

    fn is_less_and_take_subtract(
        k: i32,
        n: i32,
        groups: &[i32; 26],
        primes: &Vec<i32>,
        log_fact: &Vec<f64>,
    ) -> Option<i32> {
        if k <= 1 {
            return None;
        }

        let mut expected_log = log_fact[n as usize];
        for &group in groups.iter() {
            expected_log -= log_fact[group as usize];
        }
        if expected_log > 9.0 {
            return None;
        }

        let mut primes_count: HashMap<i32, i32> = HashMap::new();

        for &prime in primes.iter() {
            if prime > n {
                break;
            }
            primes_count.insert(prime, 0);
            let mut divisor = prime;
            while divisor <= n {
                *primes_count.get_mut(&prime).unwrap() += n / divisor;
                divisor *= prime;
            }
        }
        for &group in groups.iter() {
            if group == 0 {
                continue;
            }
            for &prime in primes.iter() {
                if prime > group {
                    break;
                }
                let mut divisor = prime;
                while divisor <= group {
                    *primes_count.get_mut(&prime).unwrap() -= group / divisor;
                    divisor *= prime;
                }
            }
        }

        let mut combination = 1i64;
        for (prime, mut count) in primes_count.into_iter() {
            while count > 0 {
                combination *= prime as i64;
                if combination >= k as i64 {
                    return None;
                }
                count -= 1;
            }
        }

        Some(k - combination as i32)
    }

    fn find_primes_less_than(n: usize) -> Vec<i32> {
        let mut table = vec![true; n + 1];
        let mut result = vec![];

        for i in 2..=n {
            if table[i] {
                result.push(i as i32);
            }
            let mut multiple = i * 2;
            while multiple <= n {
                table[multiple] = false;
                multiple += i;
            }
        }

        result
    }
}

#[test]
fn test_solution() {
    Solution::smallest_palindrome("aa".to_string(), 2);
}
