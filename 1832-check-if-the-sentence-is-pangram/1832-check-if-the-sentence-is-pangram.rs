use std::collections::HashSet;
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
          let mut sentence = sentence.chars().collect::<Vec<char>>();
    let mut v = HashSet::new();
    for i in sentence {
        v.insert(i);
    }
    if v.len() <26{
        return false
    }else{
        true
    }
    }
}