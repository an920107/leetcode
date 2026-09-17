pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        let mut memo = vec![vec![vec![None; 2]; k as usize + 1]; n as usize + 1];
        ((Self::recursion(&mut memo, n, k, false) + Self::recursion(&mut memo, n, k - 1, true))
            % MOD) as i32
    }

    fn recursion(memo: &mut Vec<Vec<Vec<Option<i64>>>>, n: i32, k: i32, flag: bool) -> i64 {
        if !flag && k == 0 {
            return 1;
        } else if n <= 1 {
            return 0;
        }

        if let Some(result) = memo[n as usize][k as usize][if flag { 1 } else { 0 }] {
            return result;
        }

        let result = (if flag {
            Self::recursion(memo, n - 1, k, true) + Self::recursion(memo, n, k, false)
        } else {
            Self::recursion(memo, n - 1, k, false) + Self::recursion(memo, n - 1, k - 1, true)
        }) % MOD;
        memo[n as usize][k as usize][if flag { 1 } else { 0 }] = Some(result);
        result
    }
}
