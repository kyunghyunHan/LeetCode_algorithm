impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let w1 =word1.chars().collect::<Vec<char>>();
        let w2 =word2.chars().collect::<Vec<char>>();
        let max= word1.len().max(w2.len());
        let mut res = "".to_string();
        for i in 0..max{
            // for j in 0..w2.len(){
             if w1.len() > i{
                res+=&w1[i].to_string();
             }
             if w2.len() > i{
                res+=&w2[i].to_string();
             }

            // }
        }
        res
    }
}