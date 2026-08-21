pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        let mut lcms: Vec<HashMap<u32, u64>> = vec![HashMap::new(); coins.len() + 1];
        let mut mask = 1u32;
        let mask_limit = (1 << coins.len()) - 1;
        while mask <= mask_limit {
            let mut ones: Vec<usize> = Vec::with_capacity(coins.len());
            for i in 0..coins.len() {
                if (1 << i) & mask > 0 {
                    ones.push(i);
                }
            }

            if ones.len() == 1 {
                lcms[1].insert(mask, coins[ones[0]] as u64);
            } else if ones.len() == 2 {
                let a = coins[ones[0]] as u64;
                let b = coins[ones[1]] as u64;
                let gcd = Self::gcd(a, b);
                lcms[2].insert(mask, a / gcd * b);
            } else {
                let a = *lcms[ones.len() - 1]
                    .get(&(mask ^ (1u32 << *ones.last().unwrap())))
                    .unwrap();
                let b = coins[*ones.last().unwrap()] as u64;
                let gcd = Self::gcd(a, b);
                lcms[ones.len()].insert(mask, a / gcd * b);
            }

            mask += 1;
        }

        let mut left = 1u64;
        let mut right = *coins.iter().min().unwrap() as u64 * k as u64;
        while left < right {
            let mid = (left + right) / 2;
            if Self::check(&lcms, k, mid) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left as i64
    }

    fn gcd(a: u64, b: u64) -> u64 {
        if a == 0 { b } else { Self::gcd(b % a, a) }
    }

    fn check(lcms: &Vec<HashMap<u32, u64>>, k: i32, target: u64) -> bool {
        let mut comb = 0i64;

        for (i, mask_lcm_map) in lcms.iter().enumerate() {
            let mut acc = 0;
            for (_, &lcm) in mask_lcm_map.iter() {
                acc += target / lcm;
            }
            comb += if i % 2 == 1 {
                acc as i64
            } else {
                -(acc as i64)
            };
        }

        comb < k as i64
    }
}

#[test]
fn test_sol() {
    let mut v = vec![1, 2, 4];
    v.push(8);
    dbg!(v);
}
