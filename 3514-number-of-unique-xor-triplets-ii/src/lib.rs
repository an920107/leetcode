pub struct Solution;

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let max_num = 2usize.pow(nums.iter().max().copied().unwrap().ilog2() + 1);

        let mut xor_stage_1_seen = vec![false; max_num];
        for i in 0..n {
            for j in i..n {
                xor_stage_1_seen[nums[i] as usize ^ nums[j] as usize] = true;
            }
        }

        let mut xor_stage_2_seen = vec![false; max_num];
        for xor_num in xor_stage_1_seen
            .into_iter()
            .enumerate()
            .filter(|(_, v)| *v)
            .map(|(k, _)| k)
        {
            for num in nums.iter() {
                xor_stage_2_seen[xor_num ^ *num as usize] = true;
            }
        }

        xor_stage_2_seen.into_iter().filter(|v| *v).count() as i32
    }
}
