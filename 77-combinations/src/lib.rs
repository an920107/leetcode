pub struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        (1u32..(1u32 << n as u32))
            .into_iter()
            .filter(|b| b.count_ones() == k as u32)
            .map(Solution::get_nums_from_binary)
            .collect()
    }

    fn get_nums_from_binary(mut b: u32) -> Vec<i32> {
        let mut result = vec![];

        let mut num = 1;
        while b > 0 {
            if b & 1 == 1 {
                result.push(num);
            }
            b >>= 1;
            num += 1;
        }

        result
    }
}
