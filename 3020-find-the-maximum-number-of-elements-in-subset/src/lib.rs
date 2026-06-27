pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut bases: HashMap<i32, HashMap<i64, i32>> = HashMap::new();

        for &num in nums.iter() {
            *bases.entry(num).or_default().entry(num as i64).or_default() += 1;
            if num == 1 {
                continue;
            }
            let mut current_num = num;
            loop {
                let num_sqrt = current_num.isqrt();
                if num_sqrt * num_sqrt == current_num {
                    *bases
                        .entry(num_sqrt)
                        .or_default()
                        .entry(num as i64)
                        .or_default() += 1;
                } else {
                    break;
                }
                current_num = num_sqrt;
            }
        }

        let mut result = 0;

        for (&base, counts) in bases.iter() {
            let mut base = base as i64;

            if base == 1
                && let Some(&count) = counts.get(&1)
            {
                result = result.max(if count % 2 == 1 { count } else { count - 1 });
                continue;
            }

            let mut current_result = 0;
            loop {
                if let Some(&count) = counts.get(&base) {
                    if count > 0 {
                        current_result += 1;
                    }
                    if count < 2 {
                        break;
                    }
                } else {
                    break;
                }
                base *= base;
            }
            result = result.max(current_result * 2 - 1);
        }

        result
    }
}

#[test]
fn test_feature() {
    let input = vec![14, 14, 196, 196, 38416, 38416];
    assert_eq!(Solution::maximum_length(input), 5);
}
