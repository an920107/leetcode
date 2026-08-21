pub struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut arr_1 = Vec::with_capacity(nums.len());
        let mut arr_2 = Vec::with_capacity(nums.len());

        arr_1.push(nums[0]);
        arr_2.push(nums[1]);

        for num in nums.into_iter().skip(2) {
            if arr_1.last().unwrap() > arr_2.last().unwrap() {
                arr_1.push(num);
            } else {
                arr_2.push(num);
            }
        }

        [arr_1, arr_2].concat()
    }
}
