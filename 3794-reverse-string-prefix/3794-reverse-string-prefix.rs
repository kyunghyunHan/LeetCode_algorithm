impl Solution {
    pub fn reverse_prefix(s: String, k: i32) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        let k = k as usize;
        let mut res = "".to_string();

        for i in (0..k).rev(){
            println!("{}",i);
            res+=&(s[i].to_string());
        }
        
        for j in k..s.len(){
            res+=&s[j].to_string();
        }
        res
    }
}