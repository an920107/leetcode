pub struct Solution;

impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let mut s1 = s1.as_bytes().to_vec();
        let mut s2 = s2.as_bytes().to_vec();

        Self::minify_string(&mut s1);
        Self::minify_string(&mut s2);

        s1 == s2
    }

    fn minify_string(s: &mut Vec<u8>) {
        if s[0] > s[2] {
            s.swap(0, 2);
        }
        if s[1] > s[3] {
            s.swap(1, 3);
        }
    }
}
