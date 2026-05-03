pub struct Solution;

impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        let (bytes1_even, bytes1_odd) = Self::partition(&s1);
        let (bytes2_even, bytes2_odd) = Self::partition(&s2);

        let freq1_even = Self::count_freq(&bytes1_even);
        let freq1_odd = Self::count_freq(&bytes1_odd);
        let freq2_even = Self::count_freq(&bytes2_even);
        let freq2_odd = Self::count_freq(&bytes2_odd);

        freq1_even == freq2_even && freq1_odd == freq2_odd
    }

    fn partition(s: &str) -> (Vec<u8>, Vec<u8>) {
        let mut bytes_even: Vec<u8> = vec![];
        let mut bytes_odd: Vec<u8> = vec![];
        for (index, c) in s.bytes().enumerate() {
            if index % 2 == 0 {
                bytes_even.push(c);
            } else {
                bytes_odd.push(c);
            }
        }
        (bytes_even, bytes_odd)
    }

    fn count_freq(bytes: &[u8]) -> [usize; 26] {
        let mut result = [0; 26];
        for c in bytes {
            result[(*c - b'a') as usize] += 1;
        }
        result
    }
}
