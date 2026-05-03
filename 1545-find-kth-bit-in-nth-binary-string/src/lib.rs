pub struct Solution;

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        if n == 1 {
            return '0';
        }

        let mid = 1 << (n - 1);
        if k == mid {
            return '1';
        }

        if k < mid {
            Solution::find_kth_bit(n - 1, k)
        } else {
            let mirror = mid * 2 - k;
            let bit = Solution::find_kth_bit(n - 1, mirror);
            if bit == '1' { '0' } else { '1' }
        }
    }
}
