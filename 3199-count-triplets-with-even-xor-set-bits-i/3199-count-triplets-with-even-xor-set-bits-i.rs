impl Solution {
    pub fn triplet_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>) -> i32 {
        let mut cnt = 0;
        for i in 0..a.len(){
            for j in 0..b.len(){
               for k in 0..c.len(){
                 let xor  = a[i] ^ b[j] ^ c[k];
                 let mut set_bits = 0;
                 let binary = format!("{:b}", xor);
                  let set_bits = xor.count_ones(); 
                if set_bits % 2 == 0 {
                    cnt += 1;
                }
               }
            }
        }
        cnt
    }
}