use std::collections::HashSet;

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut min  = 0;
        let mut set = HashSet::new();
        for c in s.chars(){
               if !set.insert(c) {
                return c;
            }
        }
        unreachable!()

        
    }
}