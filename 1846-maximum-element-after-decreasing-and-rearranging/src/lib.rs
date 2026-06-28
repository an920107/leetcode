pub struct Solution;

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
        arr.sort();
        let mut result = 0;
        for &num in arr.iter() {
            if num > result {
                result += 1;
            }
        }
        result
    }
}
