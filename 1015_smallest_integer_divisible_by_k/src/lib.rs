pub struct Solution;

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        if k % 2 == 0 || k % 5 == 0 {
            return -1;
        }

        let mut set = std::collections::HashSet::<i32>::new();

        let mut d = 0;
        let mut len = 1;
        loop {
            d = (d * 10 + 1) % k;
            if d == 0 {
                break;
            }

            if set.contains(&d) {
                return -1;
            }
            set.insert(d);
            len += 1;
        }

        len
    }
}
