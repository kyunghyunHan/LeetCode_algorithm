impl Solution {
   pub fn anagram_mappings(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    nums1
        .iter()
        .map(|num| nums2.iter().position(|x| x == num).unwrap() as i32)
        .collect()
}
}