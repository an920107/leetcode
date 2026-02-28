pub struct Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        Solution::recursion(&s)
    }

    fn recursion(s: &str) -> i32 {
        if s == "1" {
            return 0;
        }

        let last_bit = s.bytes().rev().next().unwrap_or(b'0');
        if last_bit == b'0' {
            if s.len() > 1 {
                Solution::recursion(&s[0..s.len() - 1]) + 1
            } else {
                Solution::recursion(&"0") + 1
            }
        } else {
            let mut new_bits: Vec<char> = vec![];
            let mut carry = true;
            for c in s.bytes().rev() {
                let bit = c == b'1';
                new_bits.push(if carry ^ bit { '1' } else { '0' });
                carry &= bit;
            }
            if carry {
                new_bits.push('1');
            }
            new_bits.reverse();
            Solution::recursion(&new_bits.iter().collect::<String>()) + 1
        }
    }
}
