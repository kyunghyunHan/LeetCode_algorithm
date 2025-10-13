impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut ans = 0;
        for i in 0..s.len(){
         let val = (b'z' - s[i] as u8 + 1) as i32;
         
          println!("{}",val * (i +1) as i32);
          ans +=val * (i +1) as i32;
        }
        ans
    }
}