pub struct Solution;

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        if nums.is_sorted() {
            return true;
        }

        let mut drop_count = 0;
        let mut prev_num = nums[0];
        for &current_num in nums.iter().skip(1) {
            if current_num < prev_num {
                drop_count += 1;
                if drop_count > 1 {
                    return false;
                }
            }
            prev_num = current_num;
        }
        if nums[0] < nums[nums.len() - 1] {
            return false;
        }

        true
    }
}

// 把整個 array 視為一個環
//
// impl Solution {
//     pub fn check(black_nums: Vec<i32>) -> bool {
//         black_nums
//             .iter()
//             .enumerate()
//             .filter(|&(black_i, &black_x)| black_x > black_nums[(black_i + 1) % black_nums.len()])
//             .count()
//             <= 1
//     }
// }
