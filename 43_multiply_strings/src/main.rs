use std::cmp;

fn main() {
    let x = "000".to_string();
    let y = "0".to_string();
    println!("{}", Solution::plus(x, y));
}

struct Solution;

impl Solution {
    pub fn multiply(x: String, y: String) -> String {
        Solution::karatsuba(&x, &y)
    }

    fn karatsuba(x: &str, y: &str) -> String {
        if x.len() <= 9 && y.len() <= 9 {
            return (x.parse::<i64>().unwrap() * y.parse::<i64>().unwrap()).to_string();
        }

        let len = cmp::max(x.len(), y.len());
        let len_half = len / 2;

        let mut x_string = x.to_string();
        if x.len() < len {
            let zeros = "0".repeat(len - x.len());
            x_string = zeros + &x;
        }
        let mut y_string = y.to_string();
        if y.len() < len {
            let zeros = "0".repeat(len - y.len());
            y_string = zeros + &y;
        }

        let x1 = &x_string[..len_half];
        let x2 = &x_string[len_half..];
        let y1 = &y_string[..len_half];
        let y2 = &y_string[len_half..];

        let a = Solution::karatsuba(x1, y1);
        let b = Solution::karatsuba(x2, y2);
        let c = Solution::karatsuba(x1, y2);
        let d = Solution::karatsuba(x2, y1);

        let result = Solution::plus(a + &"0".repeat((len - len_half) * 2), b);
        let result = Solution::plus(result, c + &"0".repeat(len - len_half));
        let result = Solution::plus(result, d + &"0".repeat(len - len_half));

        result
    }

    fn plus(x: String, y: String) -> String {
        let len = cmp::max(x.len(), y.len());

        let x_chars_rev = x.chars().rev().collect::<Vec<char>>();
        let y_chars_rev = y.chars().rev().collect::<Vec<char>>();

        let mut carry = 0;
        let mut result = String::new();

        for i in 0..len {
            let xn = if i < x.len() { x_chars_rev[i] } else { '0' } as i32 - 48;
            let yn = if i < y.len() { y_chars_rev[i] } else { '0' } as i32 - 48;

            let sum = xn + yn + carry;
            carry = sum / 10;
            result.push_str(&(sum % 10).to_string());
        }

        if carry > 0 {
            result.push_str(&carry.to_string());
        }

        let mut start_index = -1;
        for (i, c) in result.chars().rev().enumerate() {
            if c != '0' {
                start_index = i as i32;
                break;
            }
        }

        if start_index < 0 {
            "0".to_string()
        } else {
            result.chars().rev().collect::<Vec<char>>()[start_index as usize..]
                .iter()
                .collect()
        }
    }
}
