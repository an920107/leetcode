pub struct Solution;

impl Solution {
    pub fn max_sum_div_three(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        let mut sum = 0;
        let mut buffer_1 = vec![];
        let mut buffer_2 = vec![];

        for num in nums {
            sum += num;
            if num % 3 == 1 {
                buffer_1.push(num);
            } else if num % 3 == 2 {
                buffer_2.push(num);
            }
        }

        match sum % 3 {
            0 => sum,
            1 => {
                if buffer_1.len() < 1 && buffer_2.len() < 2 {
                    return 0;
                }

                let &a = buffer_1.get(0).unwrap_or(&100_000);
                let &b_1 = buffer_2.get(0).unwrap_or(&100_000);
                let &b_2 = buffer_2.get(1).unwrap_or(&100_000);

                sum - a.min(b_1 + b_2)
            }
            2 => {
                if buffer_2.len() < 1 && buffer_1.len() < 2 {
                    return 0;
                }

                let &a = buffer_2.get(0).unwrap_or(&100_000);
                let &b_1 = buffer_1.get(0).unwrap_or(&100_000);
                let &b_2 = buffer_1.get(1).unwrap_or(&100_000);

                sum - a.min(b_1 + b_2)
            }
            _ => 0,
        }
    }
}
