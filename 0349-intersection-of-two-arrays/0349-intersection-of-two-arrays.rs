impl Solution {
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut v = Vec::new();

    for i in nums1 {
        if nums2.contains(&i) {
            if !v.contains(&i) {
                v.push(i);
            }
        }
    }
    v
}

}