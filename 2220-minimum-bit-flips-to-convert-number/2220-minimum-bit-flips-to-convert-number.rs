impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
       let mut ans = start ^ goal;
       let mut count =0;
       while ans > 0{
        count+=1;
        ans = ans & (ans-1);
       }
       return count
    }
}