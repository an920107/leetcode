pub struct Solution;

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut groups: Vec<Vec<usize>> = vec![vec![]; n + 1];
        for (index, num) in nums.into_iter().enumerate() {
            groups[num as usize].push(index);
        }

        let mut distance = i32::MAX;
        for group in groups.into_iter() {
            if group.len() < 3 {
                continue;
            }

            for i in 0..(group.len() - 2) {
                distance = distance.min(Self::count_distance(group[i], group[i + 2]) as i32);
            }
        }

        if distance == i32::MAX { -1 } else { distance }
    }

    pub fn count_distance(i: usize, k: usize) -> usize {
        2 * (k - i)
    }
}
