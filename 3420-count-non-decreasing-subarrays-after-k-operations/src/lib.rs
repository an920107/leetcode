pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn count_non_decreasing_subarrays(mut nums: Vec<i32>, k: i32) -> i64 {
        let k = k as i64;
        let mut queue: VecDeque<QueueParams> = VecDeque::new();

        nums.reverse();

        let mut result = 0;
        let mut cost = 0;

        let mut left = 0;
        for right in 0..nums.len() {
            queue.push_back(QueueParams {
                num: nums[right],
                count: 1,
            });
            while let Some(most_back) = queue.pop_back() {
                if let Some(second_back) = queue.pop_back() {
                    if most_back.num < second_back.num {
                        queue.push_back(second_back);
                        queue.push_back(most_back);
                        break;
                    } else {
                        queue.push_back(QueueParams {
                            num: most_back.num,
                            count: most_back.count + second_back.count,
                        });
                        cost += (most_back.num - second_back.num) as i64 * second_back.count as i64;
                    }
                } else {
                    queue.push_back(most_back);
                    break;
                }
            }

            while cost > k {
                if let Some(front) = queue.pop_front() {
                    cost -= front.num as i64 - nums[left] as i64;
                    if front.count > 1 {
                        queue.push_front(QueueParams {
                            num: front.num,
                            count: front.count - 1,
                        });
                    }
                }

                left += 1;
            }

            result += right - left + 1;
        }

        result as i64
    }
}

struct QueueParams {
    num: i32,
    count: usize,
}
