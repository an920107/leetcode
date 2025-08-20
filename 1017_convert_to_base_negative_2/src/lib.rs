pub struct Solution;

impl Solution {
    pub fn base_neg2(mut n: i32) -> String {
        let mut rems: Vec<char> = vec![];

        loop {
            let mut r = n % -2;
            n /= -2;

            //    n * Q + r = n * (Q + x) + (r - n)
            // => x = 1
            if r < 0 {
                r += 2;
                n += 1;
            }

            rems.push(if r == 0 { '0' } else { '1' });

            if n == 0 {
                break;
            }
        }

        rems.iter().rev().collect()
    }
}
