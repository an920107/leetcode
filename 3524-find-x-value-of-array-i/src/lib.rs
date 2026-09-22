pub struct Solution;

impl Solution {
    pub fn result_array(mut nums: Vec<i32>, k: i32) -> Vec<i64> {
        nums.iter_mut().for_each(|n| *n %= k);
        let mut memo = vec![vec![None; k as usize]; nums.len()];
        (0..k)
            .map(|r| Self::result_until_index(&mut memo, &nums, k, nums.len() - 1, r).total)
            .collect()
    }

    fn result_until_index(
        memo: &mut Vec<Vec<Option<SolOutput>>>,
        nums: &[i32],
        k: i32,
        i: usize,
        r: i32,
    ) -> SolOutput {
        if let Some(result) = memo[i][r as usize] {
            return result;
        }

        let mut result = SolOutput::default();

        if nums[i] == r {
            result.available_for_next += 1;
            result.total += 1;
        }

        if i == 0 {
            return result;
        }

        for last_r in 0..k {
            let last_result = Self::result_until_index(memo, nums, k, i - 1, last_r);

            if last_r == r {
                result.total += last_result.total;
            }

            if last_r * nums[i] % k == r {
                result.available_for_next += last_result.available_for_next;
                result.total += last_result.available_for_next;
            }
        }

        memo[i][r as usize] = Some(result);
        result
    }
}

#[derive(Clone, Copy, Default)]
struct SolOutput {
    available_for_next: i64,
    total: i64,
}
