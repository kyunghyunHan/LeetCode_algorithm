impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    let mut v = Vec::new();

    let mut i = 0;
    let mut j = 0;
    nums1.sort_by(|a, b| a.cmp(&b));
    nums2.sort_by(|a, b| a.cmp(&b));

    loop {
        if i >= nums1.len() || j >= nums2.len() {
            break;
        }
        if nums1[i] == nums2[j] {
            v.push(nums1[i]);
            i += 1;
            j += 1;
        } else if nums1[i] < nums2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    v
    }
}