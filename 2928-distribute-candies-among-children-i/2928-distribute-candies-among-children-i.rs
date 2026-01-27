impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        let mut cnt = 0;

        for i in 0..=limit{
            for j in 0..=limit{
                for k in 0..=limit{
                     if i+k+j==n{
                        cnt+=1;
                     }
                }
            }

        }
        cnt
    }

}