impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        let mut result = "".to_string();

        for word in words.iter(){
            let mut sum:usize = 0;

            for c in word.chars(){
              sum+=  weights[c as usize - 97] as usize;
            }
             let m = (sum % 26) as u8;

            let mapped = (b'z' - m) as char;
            
            result.push(mapped);
        }
        result
    }
}