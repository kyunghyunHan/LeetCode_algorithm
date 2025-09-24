impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut r1 = vec![];
        let mut r2 = vec![];

        for num in &nums1{
            if !nums2.contains(num){
                if !r1.contains(num){
                   r1.push(*num);
                }
            }
        }
          for num in &nums2{
            if !nums1.contains(num){
              if !r2.contains(num){
                   r2.push(*num);
                }            }
        }
        vec![r1,r2]
    }
}