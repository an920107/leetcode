pub struct Solution;

impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let factors = Self::find_factors(nums.len());

        'factor_loop: for &factor in factors.iter() {
            let mut last_max_num = i32::MIN;
            for chunk in nums.chunks(factor) {
                if !Self::is_sortable(&chunk) {
                    continue 'factor_loop;
                }
                let (mut min_num, mut max_num) = (i32::MAX, i32::MIN);
                for &num in chunk {
                    min_num = min_num.min(num);
                    max_num = max_num.max(num);
                }
                if min_num < last_max_num {
                    continue 'factor_loop;
                }
                last_max_num = max_num;
            }
            sum += factor;
        }

        sum as i32
    }

    fn find_factors(n: usize) -> Vec<usize> {
        let sqrt_n = n.isqrt();
        let mut f = 1;
        let mut l_factors = vec![];
        let mut r_factors = vec![];
        while f <= sqrt_n {
            if n % f == 0 {
                l_factors.push(f);
                if n / f != f {
                    r_factors.push(n / f);
                }
            }
            f += 1;
        }
        r_factors.reverse();
        [l_factors, r_factors].concat()
    }

    fn is_sortable(nums: &[i32]) -> bool {
        let mut decreasing_count = 0;
        for window in nums.windows(2) {
            if window[1] < window[0] {
                decreasing_count += 1;
                if decreasing_count >= 2 {
                    return false;
                }
            }
        }

        if decreasing_count == 0 {
            return true;
        }

        nums[nums.len() - 1] <= nums[0]
    }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::sortable_integers(vec![5, 8]), 3);
}

#[test]
fn test_find_factors() {
    assert_eq!(Solution::find_factors(12), vec![1, 2, 3, 4, 6, 12]);
    assert_eq!(Solution::find_factors(16), vec![1, 2, 4, 8, 16]);
}
