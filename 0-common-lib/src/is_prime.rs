// Miller-Rabin Algorithm
const A_POOL: [i64; 7] = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];

pub fn is_prime(n: i64) -> bool {
    if n == 2 {
        return true;
    } else if n & 1 == 0 {
        return false;
    }
    let (d, s) = find_d_and_s(n);
    'a_pool_iter: for &a in A_POOL.iter() {
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue 'a_pool_iter;
        }
        for _ in 0..(s - 1) {
            x = (x as i128 * x as i128 % n as i128) as i64;
            if x == n - 1 {
                continue 'a_pool_iter;
            }
        }
        return false;
    }
    true
}

fn find_d_and_s(n: i64) -> (i64, i64) {
    let mut d = n - 1;
    let mut s = 0;
    while d & 1 == 0 {
        s += 1;
        d >>= 1;
    }
    (d, s)
}

fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1i64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = ((result as i128 * base as i128) % (modulus as i128)) as i64;
        }
        base = ((base as i128 * base as i128) % (modulus as i128)) as i64;
        exp >>= 1;
    }
    result
}
