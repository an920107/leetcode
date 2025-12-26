/*
 * @lc app=leetcode id=3074 lang=rust
 *
 * [3074] Apple Redistribution into Boxes
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        let mut apples = apple.iter().sum::<i32>();

        capacity.sort();
        let mut result = 0;
        for size in capacity.iter().rev() {
            apples -= size;
            result += 1;
            if apples <= 0 {
                break;
            }
        }

        result
    }
}
// @lc code=end
