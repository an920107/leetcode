pub struct Solution;

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        arr1.into_iter()
            .filter(|&num1| arr2.iter().all(|&num2| num1.abs_diff(num2) > d as u32))
            .count() as i32
    }
}
