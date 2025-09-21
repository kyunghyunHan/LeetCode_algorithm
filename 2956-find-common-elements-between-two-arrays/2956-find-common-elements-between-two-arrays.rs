impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result =vec![0,0];
          for num in &nums1{
            if nums2.contains(&num){
                result[0]+=1;
            }
          }
           for num in &nums2{
            if nums1.contains(&num){
                result[1]+=1;
            }
          }
          result
    }   
}