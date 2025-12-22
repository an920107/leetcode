pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
        let n = prices.len();

        let profits: Vec<i32> = prices
            .iter()
            .zip(strategy.iter())
            .map(|(&p, &s)| p * s)
            .collect();

        let mut profits_prefix: Vec<i64> = vec![0];
        for p in profits {
            profits_prefix.push(profits_prefix.last().unwrap() + p as i64);
        }

        let mut prices_prefix: Vec<i64> = vec![0];
        for p in prices {
            prices_prefix.push(prices_prefix.last().unwrap() + p as i64);
        }

        let mut result = *profits_prefix.last().unwrap();

        let mut left = 0;
        let mut right = k as usize - 1;
        while right < n {
            let left_original_profit = profits_prefix[left];
            let right_original_profit = profits_prefix[n] - profits_prefix[right + 1];

            let half_k_index = (left + right) / 2;
            let last_half_k_modified_profit =
                prices_prefix[right + 1] - prices_prefix[half_k_index + 1];

            result = result.max(left_original_profit + right_original_profit + last_half_k_modified_profit);

            left += 1;
            right += 1;
        }

        result
    }
}
