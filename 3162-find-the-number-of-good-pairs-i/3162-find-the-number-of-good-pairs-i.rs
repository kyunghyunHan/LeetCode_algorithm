impl Solution {
 pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for &a in &nums1 {
            for &b in &nums2 {
                if a % (b * k) == 0 {
                    count += 1;
                }
            }
        }
        count
    }
}