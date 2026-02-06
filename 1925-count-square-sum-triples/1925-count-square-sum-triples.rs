impl Solution {
    pub fn count_triples(n: i32) -> i32 {
         let n = n as usize;
         let mut sq = std::collections::HashSet::with_capacity(n);

         for c in 1..=n{
            sq.insert((c*c) as i32);
         }

         let mut ans = 0;
         for a in 1..=n{
            let a2 = (a*a) as i32;

            for b in 1..=n{
                let sum = a2 + (b*b)as i32;
                if sq.contains(&sum){
                    ans +=1;
                }
            }
         }
         ans
    }
}