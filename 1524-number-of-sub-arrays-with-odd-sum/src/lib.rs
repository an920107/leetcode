pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut odd_count = 0;
        let mut even_count = 0;

        let mut prefix_sum = Vec::with_capacity(arr.len() + 1);
        prefix_sum.push(0);

        for &num in arr.iter() {
            let v = (prefix_sum.last().unwrap() + num) % 2;
            prefix_sum.push(v);
            if v == 1 {
                odd_count += 1;
            } else {
                even_count += 1;
            }
        }

        let mut result = 0;
        for window in prefix_sum.windows(2) {
            let (current, next) = (window[0], window[1]);
            result = (result + if current == 1 { even_count } else { odd_count }) % MOD;
            if next == 1 {
                odd_count -= 1;
            } else {
                even_count -= 1;
            }
        }

        result as i32
    }
}
