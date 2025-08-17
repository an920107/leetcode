use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut numbers_shown = HashSet::<i32>::new();
        numbers_shown.insert(n);

        while n != 1 {
            let mut new_n = 0;
            for c in n.to_string().chars() {
                new_n += (c as i32 - 48).pow(2);
            }
            n = new_n;

            if numbers_shown.contains(&n) {
                return false;
            }
            numbers_shown.insert(n);
        }

        true
    }
}
