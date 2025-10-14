impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let mut a =0;
        let mut b = 0;
        for num in nums{
          if num < 10 {
            a+=num;
          }else{
            b+=num;
          }
        }
        a!=b
    }
}