fn main() {}

struct Solution;

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut num_chars = num.to_string().chars().collect::<Vec<char>>();
        for (i, c) in num_chars.clone().iter().enumerate() {
            if *c == '6' {
                num_chars[i] = '9';
                break;
            }
        }

        num_chars
            .into_iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap()
    }
}
