impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
         
         let mut cnt = 0;
    for s in patterns {
        if word.contains(&s) {
            cnt += 1;
        }
    }
    cnt
    }
}