pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        let mut memo = vec![None; n as usize + 1];
        Self::min_cost_to_n(&mut memo, &costs, n as usize)
    }

    fn min_cost_to_n(memo: &mut Vec<Option<i32>>, costs: &[i32], n: usize) -> i32 {
        if n == 0 {
            return 0;
        }

        if let Some(result) = memo[n] {
            return result;
        }

        let mut result = Self::min_cost_to_n(memo, costs, n - 1) + costs[n - 1] + 1;

        if n >= 2 {
            result = result.min(Self::min_cost_to_n(memo, costs, n - 2) + costs[n - 1] + 4);
        }

        if n >= 3 {
            result = result.min(Self::min_cost_to_n(memo, costs, n - 3) + costs[n - 1] + 9);
        }

        memo[n] = Some(result);
        result
    }
}
