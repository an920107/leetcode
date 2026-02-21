pub struct Solution;

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let primes = vec![31, 29, 23, 19, 17, 13, 11, 7, 5, 3, 2];
        let mut is_prime: [bool; 33] = [false; 33];
        primes.iter().for_each(|&prime| is_prime[prime] = true);

        (left..=right)
            .map(|n| n.count_ones())
            .filter(|&cnt| is_prime[cnt as usize])
            .count() as i32
    }
}
