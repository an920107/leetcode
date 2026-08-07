// Pollard's rho Algorithm

use std::collections::HashMap;

use crate::{gcd::gcd, is_prime::is_prime};

pub fn prime_factorization(mut n: i64) -> HashMap<i64, i32> {
    let mut result = HashMap::new();

    while !is_prime(n) {
        let p = sub_prime_factorization(n);
        *result.get_mut(&p).copied().get_or_insert_default() += 1;
        n /= p;
    }

    result
}

pub fn sub_prime_factorization(n: i64) -> i64 {
    let mut c = (rand::random::<u32>() as i64 + 1) / (n - 1) + 1;
    let mut x = 2i64;
    let mut y = 2i64;
    loop {
        x = ((x as i128 * x as i128 + c as i128) % n as i128) as i64;
        y = ((y as i128 * y as i128 + c as i128) % n as i128) as i64;
        y = ((y as i128 * y as i128 + c as i128) % n as i128) as i64;
        let d = gcd(x.abs_diff(y), n as u64) as i64;
        if 1 < d && d < n {
            return if is_prime(d) {
                d
            } else {
                sub_prime_factorization(d)
            };
        } else if d == n {
            c = (rand::random::<u32>() as i64 + 1) / (n - 1) + 1;
            x = 2i64;
            y = 2i64;
        }
    }
}
