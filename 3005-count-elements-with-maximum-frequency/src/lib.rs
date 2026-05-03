pub struct Solution;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::<i32, i32>::new();

        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        let mut max_freq = 0;
        let mut max_freq_count = 0;
        for freq in map.values() {
            if *freq > max_freq {
                max_freq = *freq;
                max_freq_count = 1;
            } else if *freq == max_freq {
                max_freq_count += 1;
            }
        }

        max_freq_count * max_freq
    }
}
