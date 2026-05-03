pub struct Solution;

impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let mut memo: Vec<Vec<Vec<Option<i64>>>> =
            vec![vec![vec![None; 3]; k as usize + 1]; prices.len()];

        Solution::profit(prices.len() as i32 - 1, k, 0, &prices, &mut memo)
    }

    fn profit(
        day: i32,
        k: i32,
        state: usize,
        prices: &[i32],
        memo: &mut Vec<Vec<Vec<Option<i64>>>>,
    ) -> i64 {
        if k == 0 {
            return 0;
        }

        if day == 0 {
            return match state {
                0 => 0,
                1 => -prices[0] as i64,
                2 => prices[0] as i64,
                _ => 0,
            };
        }

        if let Some(profit) = memo[day as usize][k as usize][state] {
            return profit;
        }

        let profit = match state {
            0 => {
                let from_0 = Solution::profit(day - 1, k, 0, prices, memo);
                let from_1 =
                    Solution::profit(day - 1, k, 1, prices, memo) + prices[day as usize] as i64;
                let from_2 =
                    Solution::profit(day - 1, k, 2, prices, memo) - prices[day as usize] as i64;
                from_0.max(from_1).max(from_2)
            }
            1 => {
                let from_0 =
                    Solution::profit(day - 1, k - 1, 0, prices, memo) - prices[day as usize] as i64;
                let from_1 = Solution::profit(day - 1, k, 1, prices, memo);
                from_0.max(from_1)
            }
            2 => {
                let from_0 =
                    Solution::profit(day - 1, k - 1, 0, prices, memo) + prices[day as usize] as i64;
                let from_2 = Solution::profit(day - 1, k, 2, prices, memo);
                from_0.max(from_2)
            }
            _ => 0,
        };

        memo[day as usize][k as usize][state] = Some(profit);
        profit
    }
}
