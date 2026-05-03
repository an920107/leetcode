pub struct Solution;

impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        (1..=n)
            .filter(|&k| {
                let mut k = k;
                let original_k = k;
                let mut rotated_k_digs: Vec<char> = vec![];

                while k > 0 {
                    let d = k % 10;
                    if let Some(rotated_d) = Self::rotate_digit(d) {
                        rotated_k_digs.push((rotated_d as u8 + b'0') as char);
                    } else {
                        return false;
                    }
                    k /= 10;
                }

                let rotated_k = String::from_iter(rotated_k_digs.into_iter().rev())
                    .parse::<i32>()
                    .unwrap();

                rotated_k != original_k
            })
            .count() as i32
    }

    fn rotate_digit(d: i32) -> Option<i32> {
        match d {
            0 => Some(0),
            1 => Some(1),
            2 => Some(5),
            5 => Some(2),
            6 => Some(9),
            8 => Some(8),
            9 => Some(6),
            3 | 4 | 7 => None,
            _ => panic!(),
        }
    }
}
