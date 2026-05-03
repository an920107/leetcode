pub struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut sums = vec![i32::MAX; triangle.len()];
        sums[0] = triangle[0][0];

        for (layer, nums) in triangle.iter().enumerate() {
            if layer == 0 {
                continue;
            }

            let mut current_sums = vec![];
            for (index, num) in nums[..layer + 1].iter().enumerate() {
                let left = if index == 0 {
                    i32::MAX
                } else {
                    sums[index - 1]
                };

                let right = if index == layer {
                    i32::MAX
                } else {
                    sums[index]
                };

                current_sums.push(*num + std::cmp::min(left, right));
            }

            for (index, sum) in current_sums.iter().enumerate() {
                sums[index] = *sum;
            }
        }

        *sums.iter().min().unwrap()
    }
}
