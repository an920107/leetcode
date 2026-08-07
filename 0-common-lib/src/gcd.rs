pub fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 { b } else { gcd(b % a, a) }
}
