pub struct Solution;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let mut groups = std::collections::HashMap::<i32, i64>::new();

        for (index, num) in nums.iter().enumerate() {
            let index_sub_num = index as i32 - *num;
            groups
                .entry(index_sub_num)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let total_comb = Solution::count_pairs(&(nums.len() as i64));
        let same_comb: i64 = groups.values().map(Solution::count_pairs).sum();
        total_comb - same_comb
    }

    fn count_pairs(n: &i64) -> i64 {
        n * (n - 1) / 2
    }
}
