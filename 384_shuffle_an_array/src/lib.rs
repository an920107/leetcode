pub struct Solution {
    nums: Vec<i32>,
}

impl Solution {
    pub fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    pub fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }

    pub fn shuffle(&self) -> Vec<i32> {
        let mut result = self.nums.clone();
        for i in (1..result.len()).rev() {
            let j = Solution::randunder(i as u32 + 1);
            result.swap(i, j as usize);
        }
        result
    }

    fn randunder(bound: u32) -> u32 {
        let max = (std::u32::MAX / bound) * bound;
        let mut n: u32;
        loop {
            n = rand::random();
            if n < max {
                n %= bound;
                break;
            }
        }
        n
    }
}
