use std::collections::HashSet;

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
          let mut result = HashSet::new();

    for n in nums1.clone() {
        if nums2.contains(&n) {
            result.insert(n);
        }
        if nums3.contains(&n) {
            result.insert(n);
        }
    }
    for n in nums2 {
        if nums1.contains(&n) {
            result.insert(n);
        }
        if nums3.contains(&n) {
            result.insert(n);
        }
    }
        result.iter().map(|x|*x).collect::<Vec<i32>>()
        
    }
}