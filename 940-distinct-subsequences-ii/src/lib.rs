pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let mut last_seen_indices: [Option<usize>; 26] = [None; 26];
        let mut dp: Vec<i64> = Vec::with_capacity(s.len() + 1);
        dp.push(0);

        for (index, c) in s.bytes().map(|c| (c - b'a') as usize).enumerate() {
            if let Some(last_seen_index) = last_seen_indices[c] {
                let count = dp.last().copied().unwrap_or(0) * 2 - dp[last_seen_index];
                dp.push(if count < 0 { count + MOD } else { count % MOD });
            } else {
                dp.push((dp.last().copied().unwrap_or(0) * 2 + 1) % MOD);
            }
            last_seen_indices[c] = Some(index);
        }

        dp.last().copied().unwrap_or(0) as i32
    }
}
