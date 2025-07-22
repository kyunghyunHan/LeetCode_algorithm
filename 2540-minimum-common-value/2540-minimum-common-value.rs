impl Solution {
pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let nin = -1;
    for i in &nums1 {
        if nums2.contains(&i){
            return *i;
        }
    }
    nin
}

}