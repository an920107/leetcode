pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();

        let mut i = a.len() as isize - 1;
        let mut j = b.len() as isize - 1;
        let mut carry = 0;

        let mut result = String::with_capacity(a.len().max(b.len()) + 1);

        while i >= 0 || j >= 0 || carry > 0 {
            let mut sum = carry;

            if i >= 0 {
                sum += (a_bytes[i as usize] - b'0') as u32;
                i -= 1;
            }
            if j >= 0 {
                sum += (b_bytes[j as usize] - b'0') as u32;
                j -= 1;
            }

            result.push(if sum % 2 == 1 { '1' } else { '0' });
            carry = sum / 2;
        }

        result.chars().rev().collect()
    }
}
