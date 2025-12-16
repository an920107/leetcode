pub struct Solution;

impl Solution {
    pub fn get_descent_periods(mut prices: Vec<i32>) -> i64 {
        prices.push(-1);

        let mut result = 0;
        let mut start_index = 0;

        for (index, current_price) in prices.iter().enumerate().skip(1) {
            let last_price = prices[index - 1];
            if current_price - last_price != -1 {
                let period_size = index - start_index;
                result += period_size * (period_size + 1) / 2;
                start_index = index;
            }
        }

        result as i64
    }
}
