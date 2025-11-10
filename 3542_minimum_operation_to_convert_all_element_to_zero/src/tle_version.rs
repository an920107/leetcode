pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut utils = Utils::new(&nums);
        let mut result = 0;

        let mut subarray_ranges = utils.find_subarray_ranges(0, nums.len());
        while let Some(range) = subarray_ranges.pop() {
            result += 1;
            utils.substract_in_range(&range);
            subarray_ranges.extend(utils.find_subarray_ranges(range.begin, range.end));
        }

        result
    }
}

struct Range {
    begin: usize,
    end: usize,
    min_value: i32,
}

struct Utils {
    nums: Vec<i32>,
}

impl Utils {
    pub fn new(nums: &[i32]) -> Utils {
        Self {
            nums: Vec::from_iter(nums.iter().copied()),
        }
    }

    pub fn find_subarray_ranges(&self, begin: usize, end: usize) -> Vec<Range> {
        let mut result: Vec<Range> = Vec::new();
        let mut zero_state = true;

        for (index, &num) in self.nums[begin..end].iter().enumerate() {
            let index = index + begin;

            if zero_state {
                if num != 0 {
                    zero_state = false;
                    result.push(Range {
                        begin: index,
                        end: index,
                        min_value: num,
                    });
                }
            } else {
                if num == 0 {
                    zero_state = true;
                    result.last_mut().unwrap().end = index;
                } else {
                    let current_min = result.last().unwrap().min_value;
                    result.last_mut().unwrap().min_value = current_min.min(num);
                }
            }
        }

        if !zero_state {
            result.last_mut().unwrap().end = end;
        }

        result
    }

    pub fn substract_in_range(&mut self, range: &Range) {
        for num in self.nums[range.begin..range.end].iter_mut() {
            *num -= range.min_value;
        }
    }
}
