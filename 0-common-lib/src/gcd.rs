pub fn gcd(a: usize, b: usize) -> usize {
    if a == 0 { b } else { gcd(b % a, a) }
}
