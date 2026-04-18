pub struct Solution;

impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        let mut s = n.to_string().as_bytes().to_vec();
        s.reverse();
        let reversed_n = String::from_utf8(s).unwrap().parse::<i64>().unwrap();
        reversed_n.abs_diff(n as i64) as i32
    }
}
