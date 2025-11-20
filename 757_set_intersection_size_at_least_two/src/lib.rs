use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|a, b| a[1].cmp(&b[1]).then(b[0].cmp(&a[0])));

        let mut contianing_set: HashSet<i32> = HashSet::new();
        let mut last_conaining = vec![-1, -1];

        for interval in intervals {
            let mut current_conaining: Vec<i32> = Vec::new();
            let mut remains = 2;

            if last_conaining[0] >= interval[0] && last_conaining[0] <= interval[1] {
                current_conaining.push(last_conaining[0]);
                remains -= 1;
            }
            if last_conaining[1] >= interval[0] && last_conaining[1] <= interval[1] {
                current_conaining.push(last_conaining[1]);
                remains -= 1;
            }

            let mut num = interval[1];
            while remains > 0 {
                contianing_set.insert(num);
                current_conaining.push(num);
                num -= 1;
                remains -= 1;
            }
            last_conaining = current_conaining;
        }

        contianing_set.len() as i32
    }
}
