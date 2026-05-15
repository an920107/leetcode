pub struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let n = nums.len();
        let limit = limit as usize;

        let mut diff: Vec<i32> = vec![0; limit * 2 + 2];

        for i in 0..(n / 2) {
            let mut a = nums[i] as usize;
            let mut b = nums[n - i - 1] as usize;
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }

            diff[2] += 2;
            diff[a + 1] -= 1;
            diff[a + b] -= 1;
            diff[a + b + 1] += 1;
            diff[b + limit + 1] += 1;
        }

        let mut result = n as i32;
        let mut current = 0;
        for d in diff[2..].into_iter() {
            current += d;
            result = result.min(current);
        }
        result
    }
}
