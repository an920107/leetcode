pub fn fast_pow(base: u64, exp: u64, m: u64) -> u64 {
    if exp == 0 {
        1
    } else if exp & 1 == 1 {
        base * fast_pow(base * base % m, exp >> 1, m) % m
    } else {
        fast_pow(base * base % m, exp >> 1, m)
    }
}

#[test]
fn test_fast_pow() {
    const MOD: u64 = 1_000_000_007;
    assert_eq!(fast_pow(2, 30, MOD), 73741817);
    assert_eq!(fast_pow(7, 29, MOD), 593920734);
    assert_eq!(fast_pow(13, 32, MOD), 210500742);
    assert_eq!(fast_pow(20, 0, MOD), 1);
    assert_eq!(fast_pow(0, 100, MOD), 0);
}
