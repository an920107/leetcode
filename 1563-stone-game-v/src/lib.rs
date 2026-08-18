pub struct Solution;

impl Solution {
    pub fn stone_game_v(stones: Vec<i32>) -> i32 {
        let n = stones.len();

        let mut prefix = Vec::with_capacity(n + 1);
        prefix.push(0);
        for stone in stones.into_iter() {
            prefix.push(prefix.last().unwrap() + stone);
        }

        let mut memo = vec![0; (n + 1) * (n + 1)];
        Self::max_val(&prefix, &mut memo, 0, n)
    }

    fn max_val(prefix: &[i32], memo: &mut Vec<i32>, left: usize, right: usize) -> i32 {
        if left == right - 1 {
            return 0;
        } else if left == right - 2 {
            return (prefix[left + 1] - prefix[left]).min(prefix[right] - prefix[right - 1]);
        }

        if memo[left * prefix.len() + right] > 0 {
            return memo[left * prefix.len() + right];
        }

        let mut result = 0;

        for mid in (left + 1)..right {
            let left_sum = prefix[mid] - prefix[left];
            let right_sum = prefix[right] - prefix[mid];

            if left_sum < right_sum {
                result = result.max(Self::max_val(prefix, memo, left, mid) + left_sum);
            } else if left_sum > right_sum {
                result = result.max(Self::max_val(prefix, memo, mid, right) + right_sum);
            } else {
                result = result
                    .max(Self::max_val(prefix, memo, left, mid) + left_sum)
                    .max(Self::max_val(prefix, memo, mid, right) + right_sum);
            }
        }

        memo[left * prefix.len() + right] = result;
        result
    }
}
