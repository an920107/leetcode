pub struct Solution;

use std::collections::VecDeque;

const I64INF: i64 = 1125899906842624;
const I32INF: i32 = 1073741824;

impl Solution {
    pub fn max_sum_trionic(mut nums: Vec<i32>) -> i64 {
        nums.push(-I32INF);

        let mut trionic_subarrays: Vec<TrionicSubarray> = vec![];
        let mut temp: VecDeque<TempTrionicSubarray> = VecDeque::new();

        let mut last_direction = Direction::Flat;
        for i in 1..nums.len() {
            let direction = Direction::from_two(nums[i - 1], nums[i]);

            if direction == last_direction {
                continue;
            }

            match direction {
                Direction::Flat => {
                    if let Some(first) = temp.front_mut()
                        && first.q.is_some()
                    {
                        first.r = Some(i - 1);
                        trionic_subarrays.push((*first).into());
                        temp.pop_front();
                    }
                    temp.clear();
                }

                Direction::Up => {
                    if let Some(first) = temp.front_mut() {
                        first.q = Some(i - 1);
                    }

                    temp.push_back(TempTrionicSubarray {
                        l: i - 1,
                        p: None,
                        q: None,
                        r: None,
                    });
                }

                Direction::Down => {
                    for subarray in temp.iter_mut() {
                        if subarray.p.is_none() {
                            subarray.p = Some(i - 1);
                        } else {
                            subarray.r = Some(i - 1);
                        }
                    }

                    if let Some(first) = temp.front()
                        && first.r.is_some()
                    {
                        trionic_subarrays.push((*first).into());
                        temp.pop_front();
                    }
                }
            }

            last_direction = direction;
        }

        let mut result = -I64INF;

        for &TrionicSubarray { l, p, q, r } in trionic_subarrays.iter() {
            let mut left_sum = 0;
            let mut current_sum = 0i64;
            for &num in nums[l..(p - 1)].iter().rev() {
                current_sum += num as i64;
                left_sum = left_sum.max(current_sum);
            }

            let mut right_sum = 0;
            let mut current_sum = 0i64;
            for &num in nums[(q + 2)..=r].iter() {
                current_sum += num as i64;
                right_sum = right_sum.max(current_sum);
            }

            let middle_sum = nums[(p - 1)..(q + 2)]
                .iter()
                .map(|&num| num as i64)
                .sum::<i64>();
            result = result.max(left_sum + right_sum + middle_sum);
        }

        result
    }
}

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Flat,
}

impl Direction {
    fn from_two(a: i32, b: i32) -> Direction {
        if a > b {
            Direction::Down
        } else if a < b {
            Direction::Up
        } else {
            Direction::Flat
        }
    }
}

#[derive(Clone, Copy)]
struct TempTrionicSubarray {
    l: usize,
    p: Option<usize>,
    q: Option<usize>,
    r: Option<usize>,
}

#[derive(Clone, Copy)]
struct TrionicSubarray {
    l: usize,
    p: usize,
    q: usize,
    r: usize,
}

impl Into<TrionicSubarray> for TempTrionicSubarray {
    fn into(self) -> TrionicSubarray {
        TrionicSubarray {
            l: self.l,
            p: self.p.unwrap(),
            q: self.q.unwrap(),
            r: self.r.unwrap(),
        }
    }
}
