#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // key: target - num, value: index
        let mut map = std::collections::HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let sub = target - num;
            if let Some(&j) = map.get(&num) {
                return vec![j as i32, i as i32];
            }
            map.insert(sub, i);
        }
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }

    #[test]
    fn testcase_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let expected = vec![1, 2];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }

    #[test]
    fn testcase_3() {
        let nums = vec![3, 3];
        let target = 6;
        let expected = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }
}

fn main() {}
