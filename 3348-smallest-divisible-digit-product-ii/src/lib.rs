pub struct Solution;

fn digit_factors(d: u8) -> (i32, i32, i32, i32) {
    match d {
        1 => (0, 0, 0, 0),
        2 => (1, 0, 0, 0),
        3 => (0, 1, 0, 0),
        4 => (2, 0, 0, 0),
        5 => (0, 0, 1, 0),
        6 => (1, 1, 0, 0),
        7 => (0, 0, 0, 1),
        8 => (3, 0, 0, 0),
        9 => (0, 2, 0, 0),
        _ => (0, 0, 0, 0),
    }
}

fn min_digits(c2: i32, c3: i32, c5: i32, c7: i32) -> i32 {
    let c2 = c2.max(0);
    let c3 = c3.max(0);
    let c5 = c5.max(0);
    let c7 = c7.max(0);

    let q2 = c2 / 3;
    let r2 = c2 % 3;
    let q3 = c3 / 2;
    let r3 = c3 % 2;

    let rem_len = match (r2, r3) {
        (0, 0) => 0,
        (0, 1) | (1, 0) | (1, 1) | (2, 0) => 1,
        (2, 1) => 2,
        _ => unreachable!(),
    };

    c5 + c7 + q2 + q3 + rem_len
}

impl Solution {
    pub fn smallest_number(num: String, mut t: i64) -> String {
        let mut cnt2 = 0;
        let mut cnt3 = 0;
        let mut cnt5 = 0;
        let mut cnt7 = 0;

        while t % 2 == 0 {
            cnt2 += 1;
            t /= 2;
        }
        while t % 3 == 0 {
            cnt3 += 1;
            t /= 3;
        }
        while t % 5 == 0 {
            cnt5 += 1;
            t /= 5;
        }
        while t % 7 == 0 {
            cnt7 += 1;
            t /= 7;
        }

        if t > 1 {
            return "-1".to_string();
        }

        let bytes = num.as_bytes();
        let n = bytes.len();

        let mut first_zero_idx = n;
        for (i, &b) in bytes.iter().enumerate() {
            if b == b'0' {
                first_zero_idx = i;
                break;
            }
        }

        let mut pref2 = vec![0; n + 1];
        let mut pref3 = vec![0; n + 1];
        let mut pref5 = vec![0; n + 1];
        let mut pref7 = vec![0; n + 1];

        for i in 0..n {
            let (f2, f3, f5, f7) = if bytes[i] >= b'1' && bytes[i] <= b'9' {
                digit_factors(bytes[i] - b'0')
            } else {
                (0, 0, 0, 0)
            };
            pref2[i + 1] = pref2[i] + f2;
            pref3[i + 1] = pref3[i] + f3;
            pref5[i + 1] = pref5[i] + f5;
            pref7[i + 1] = pref7[i] + f7;
        }

        let m = min_digits(cnt2, cnt3, cnt5, cnt7);
        let max_p = std::cmp::min(n, first_zero_idx);

        if n >= m as usize {
            for p in (0..=max_p).rev() {
                let rem_len = (n - p) as i32;

                if p == n {
                    let r2 = cnt2 - pref2[n];
                    let r3 = cnt3 - pref3[n];
                    let r5 = cnt5 - pref5[n];
                    let r7 = cnt7 - pref7[n];

                    if min_digits(r2, r3, r5, r7) <= 0 {
                        return num;
                    }
                    continue;
                }

                let start_d = (bytes[p] - b'0' + 1) as u8;
                let rem_len_suffix = rem_len - 1;

                for d in start_d..=9 {
                    let (f2, f3, f5, f7) = digit_factors(d);
                    let r2 = cnt2 - pref2[p] - f2;
                    let r3 = cnt3 - pref3[p] - f3;
                    let r5 = cnt5 - pref5[p] - f5;
                    let r7 = cnt7 - pref7[p] - f7;

                    if min_digits(r2, r3, r5, r7) <= rem_len_suffix {
                        let mut res = Vec::with_capacity(n);
                        res.extend_from_slice(&bytes[..p]);
                        res.push(b'0' + d);

                        let mut cur2 = r2;
                        let mut cur3 = r3;
                        let mut cur5 = r5;
                        let mut cur7 = r7;

                        for step in 0..rem_len_suffix {
                            let rem_after = rem_len_suffix - 1 - step;
                            for cand in 1..=9 {
                                let (cf2, cf3, cf5, cf7) = digit_factors(cand);
                                let nr2 = cur2 - cf2;
                                let nr3 = cur3 - cf3;
                                let nr5 = cur5 - cf5;
                                let nr7 = cur7 - cf7;

                                if min_digits(nr2, nr3, nr5, nr7) <= rem_after {
                                    res.push(b'0' + cand);
                                    cur2 = nr2;
                                    cur3 = nr3;
                                    cur5 = nr5;
                                    cur7 = nr7;
                                    break;
                                }
                            }
                        }

                        return String::from_utf8(res).unwrap();
                    }
                }
            }
        }

        let target_len = std::cmp::max(n + 1, m as usize);
        let mut res = Vec::with_capacity(target_len);

        let mut cur2 = cnt2;
        let mut cur3 = cnt3;
        let mut cur5 = cnt5;
        let mut cur7 = cnt7;

        for step in 0..target_len {
            let rem_after = (target_len - 1 - step) as i32;
            for cand in 1..=9 {
                let (cf2, cf3, cf5, cf7) = digit_factors(cand);
                let nr2 = cur2 - cf2;
                let nr3 = cur3 - cf3;
                let nr5 = cur5 - cf5;
                let nr7 = cur7 - cf7;

                if min_digits(nr2, nr3, nr5, nr7) <= rem_after {
                    res.push(b'0' + cand);
                    cur2 = nr2;
                    cur3 = nr3;
                    cur5 = nr5;
                    cur7 = nr7;
                    break;
                }
            }
        }

        String::from_utf8(res).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::smallest_number("1234".to_string(), 30),
            "1235".to_string()
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::smallest_number("1234".to_string(), 55),
            "-1".to_string()
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::smallest_number("9".to_string(), 2),
            "12".to_string()
        );
    }

    #[test]
    fn test_zero_in_num() {
        assert_eq!(
            Solution::smallest_number("304".to_string(), 9),
            "313".to_string()
        );
    }

    #[test]
    fn test_t_is_1() {
        assert_eq!(
            Solution::smallest_number("1234".to_string(), 1),
            "1234".to_string()
        );
        assert_eq!(
            Solution::smallest_number("909".to_string(), 1),
            "911".to_string()
        );
    }

    #[test]
    fn test_exact_match() {
        assert_eq!(
            Solution::smallest_number("126".to_string(), 12),
            "126".to_string()
        );
    }

    #[test]
    fn test_large_number() {
        assert_eq!(
            Solution::smallest_number("999999999".to_string(), 2),
            "1111111112".to_string()
        );
    }
}
