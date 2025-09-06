impl Solution {
    pub fn pivot_integer(n: i32) -> i32{
       let mut pivot = 1;
       for i in 0..n{
               let mut lcnt = 0;
               let mut rcnt = 0;
               
          for j in 1..=pivot{
               lcnt+=j;
          }
          for k in pivot..=n{
             rcnt+=k;
          }
          if lcnt == rcnt{
            return pivot
          }
          pivot+=1;
          
       }
    return -1;


    }
}