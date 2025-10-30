use std::collections::HashSet;

impl Solution {
    pub fn minimized_string_length(s: String) -> i32 {
        let mut hs = HashSet::new();
        
        for c in s.chars(){
            hs.insert(c);
        }

        hs.len() as i32
    }
}