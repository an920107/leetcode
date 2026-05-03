#[inline]
fn rand7() -> i32 {
    (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
        % 7) as i32
}

pub struct Solution;

impl Solution {
    pub fn rand10() -> i32 {
        let mut rnd: i32;
        loop {
            rnd = (rand7() - 1) * 7 + rand7(); // [1, 49]

            if rnd <= 40 {
                break;
            }
        }
        rnd % 10 + 1
    }
}
