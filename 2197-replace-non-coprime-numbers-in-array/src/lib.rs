pub struct Solution;

impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::<i32>::new();

        for mut num in nums {
            loop {
                if stack.is_empty() {
                    stack.push(num);
                    break;
                }

                let top = *stack.last().unwrap();
                let gcd = Solution::gcd(top, num);
                if gcd == 1 {
                    stack.push(num);
                    break;
                }

                let lcm = top / gcd * num;
                num = lcm;
                stack.pop();
            }
        }

        stack
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 {
            b
        } else if a <= b {
            Solution::gcd(b % a, a)
        } else {
            Solution::gcd(b, a)
        }
    }
}
