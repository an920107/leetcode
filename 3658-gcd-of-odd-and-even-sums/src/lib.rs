pub struct Solution;

impl Solution {
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        n
    }

    // pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
    //     let n = n as usize;
    //     let odd_sum = (1..).step_by(2).take(n).sum();
    //     let even_sum = (2..).step_by(2).take(n).sum();
    //     Self::gcd(odd_sum, even_sum) as i32
    // }

    // fn gcd(a: usize, b: usize) -> usize {
    //     if a == 0 { b } else { Self::gcd(b % a, a) }
    // }
}

#[test]
fn test_sol() {
    for i in 1..=5 {
        println!("{}", Solution::gcd_of_odd_even_sums(i));
    }
}
