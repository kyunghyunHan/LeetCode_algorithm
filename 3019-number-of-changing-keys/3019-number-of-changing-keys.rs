impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut cnt =0;
        for i in 1..s.len(){
           let prev_upper:String = s[i-1].to_string().to_uppercase();
           let upper: String = s[i].to_string().to_uppercase();

           if upper !=prev_upper {
              cnt+=1;
           }

        }
        cnt
    }
}