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

            for i in 0..group.len() {
                for j in (i + 1)..group.len() {
                    for k in (j + 1)..group.len() {
                        distance =
                            distance.min(Self::count_distance(group[i], group[j], group[k]) as i32)
                    }
                }
            }
        }

        if distance == i32::MAX { -1 } else { distance }
    }

    pub fn count_distance(x: usize, y: usize, z: usize) -> usize {
        x.abs_diff(y) + x.abs_diff(z) + y.abs_diff(z)
    }
}
