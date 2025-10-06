impl Solution {
    pub fn check_string(s: String) -> bool {
      let s =s.chars().collect::<Vec<char>>();
      for i in 1..s.len(){
        if s[i-1] == 'b' && s[i]=='a'{
           return  false
        }
      }
      true
        
    }
}