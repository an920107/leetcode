pub struct Solution;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut groups = std::collections::HashMap::<i32, i64>::new();
        for n in nums {
            //    nums[i] + rev(nums[j]) == nums[j] + rev(nums[i])
            // => nums[i] - rev(nums[i]) == nums[j] - rev(nums[j])
            let num_sub_rev = n - Solution::rev(n);
            groups
                .entry(num_sub_rev)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let result: i64 = groups.values().map(|count| count * (count - 1) / 2).sum();
        (result % MOD) as i32
    }

    fn rev(mut num: i32) -> i32 {
        let mut result = 0;
        while num > 0 {
            result *= 10;
            result += num % 10;
            num /= 10;
        }
        result
    }
}
