impl Solution {
    pub fn max_distance(words: Vec<String>) -> i32 {
        let mut count = 0;
        let mut r = true;
        for i in 0..words.len(){
            for j in (i..words.len()).rev(){
                if words[i] != words[j]{
                    r = false;
                   if count < j-i{
                      count = j-i
                   }
                    
                }
            }
        }
        if r== true {
            0
        }else{
            count as i32 +1
        }
    }
}