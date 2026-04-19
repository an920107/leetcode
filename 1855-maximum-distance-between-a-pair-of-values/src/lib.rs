pub struct Solution;

impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut index1 = nums1.len().min(nums2.len()) - 1;
        let mut index2 = nums2.len() - 1;
        let mut result = if nums1[index1] <= nums2[index2] {
            index2 - index1
        } else {
            0
        };

        while index1 > 0 || index2 > 0 {
            while index1 > 0 && nums1[index1] <= nums2[index2] {
                index1 -= 1;
                if nums1[index1] <= nums2[index2] {
                    result = result.max(index2 - index1);
                }
            }
            if index1 > 0 && index1 == index2 {
                index1 -= 1;
            }

            while index2 > 0 && index1 < index2 && nums1[index1] > nums2[index2] {
                index2 -= 1;
                if nums1[index1] <= nums2[index2] {
                    result = result.max(index2 - index1);
                }
            }
            if index2 > 0 && index1 < index2 {
                index2 -= 1;
            }
        }

        result as i32
    }
}
