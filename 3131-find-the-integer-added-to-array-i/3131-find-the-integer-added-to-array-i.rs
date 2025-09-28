impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let nums1 = nums1.iter().min().unwrap();
        let nums2 = nums2.iter().min().unwrap();

        (nums2 - nums1)

    }
}