pub struct Solution;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums[0].len();
        let mut seen_nums: Vec<bool> = vec![false; 1 << n];

        for s in nums.iter() {
            let mut num = 0;
            for c in s.bytes() {
                num <<= 1;
                num |= (c == b'1') as usize;
            }
            seen_nums[num] = true;
        }

        let mut result_num = 0;
        for (num, seen) in seen_nums.iter().enumerate() {
            if !*seen {
                result_num = num;
                break;
            }
        }

        let mut result: Vec<char> = vec![];
        for _ in 0..n {
            result.push(if result_num & 1 == 1 { '1' } else { '0' });
            result_num >>= 1;
        }
        result.reverse();
        result.iter().collect()
    }
}
