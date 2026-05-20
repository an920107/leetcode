pub struct Solution;

impl Solution {
    pub fn get_common(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        let mut index1 = 0;
        let mut index2 = 0;

        nums1.push(i32::MAX);
        nums2.push(i32::MAX);

        while nums1[index1] != nums2[index2] {
            if nums1[index1] < nums2[index2] {
                index1 += 1;
            } else {
                index2 += 1;
            }
        }

        if nums1[index1] != i32::MAX {
            nums1[index1]
        } else {
            -1
        }
    }
}
