use std::collections::HashSet;
impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
                let morse = [".-","-...","-.-.","-..",".","..-.","--.","....","..",
                     ".---","-.-",".-..","--","-.","---",".--.","--.-",
                     ".-.","...","-","..-","...-",".--","-..-","-.--","--.."];
                 let mut set = HashSet::new();
                 for word in words {
                    let mut transformation = String::new();
                    for c in word.chars(){
                        let idx = (c as u8 - b'a') as usize;
                        transformation.push_str(morse[idx])
                    }
                    set.insert(transformation);
                 }
                 set.len()as i32

    }
}