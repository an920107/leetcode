pub struct Solution;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let min_jump = min_jump as usize;
        let max_jump = max_jump as usize;
        let flags: Vec<bool> = s.bytes().map(|c| c == b'1').collect();

        if flags[flags.len() - 1] {
            return false;
        }

        let mut f = vec![0; flags.len()];
        let mut prefix = vec![1; flags.len()];
        f[0] = 1;

        for index in min_jump..flags.len() {
            if flags[index] {
                prefix[index] = prefix[index - 1];
                continue;
            }

            let min_from = index as i32 - max_jump as i32;
            let max_from = index as i32 - min_jump as i32;
            let interval_sum = if min_from <= 0 {
                prefix[max_from as usize]
            } else {
                prefix[max_from as usize] - prefix[min_from as usize - 1]
            };
            f[index] = if interval_sum > 0 { 1 } else { 0 };
            prefix[index] = prefix[index - 1] + f[index];
        }

        f[flags.len() - 1] > 0
    }
}
