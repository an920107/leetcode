pub struct Solution;

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut freq: Vec<i32> = vec![0; 26];
        for c in s.bytes() {
            freq[(c - b'a') as usize] += 1;
        }
        let (vowel, consonant): (Vec<_>, Vec<_>) =
            freq.into_iter().enumerate().partition(|(index, _)| {
                let c = (*index as u8) + b'a';
                c == b'a' || c == b'e' || c == b'i' || c == b'o' || c == b'u'
            });
        vowel.into_iter().map(|(_, n)| n).max().unwrap()
            + consonant.into_iter().map(|(_, n)| n).max().unwrap()
    }
}
