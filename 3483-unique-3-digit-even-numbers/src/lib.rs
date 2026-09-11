pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let n = digits.len();

        let mut set: HashSet<i32> = HashSet::new();

        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    let num_i = digits[i];
                    let num_j = digits[j];
                    let num_k = digits[k];
                    let pool = [
                        num_i * 100 + num_j * 10 + num_k,
                        num_i * 100 + num_k * 10 + num_j,
                        num_j * 100 + num_i * 10 + num_k,
                        num_j * 100 + num_k * 10 + num_i,
                        num_k * 100 + num_i * 10 + num_j,
                        num_k * 100 + num_j * 10 + num_i,
                    ];
                    pool.iter()
                        .filter(|num| **num >= 100 && **num & 1 == 0)
                        .for_each(|num| {
                            set.insert(*num);
                        });
                }
            }
        }

        set.len() as i32
    }
}
