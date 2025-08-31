impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        let mut ms = "".to_string();
        for c in words{
            ms+=&c[0..1].to_string()
        }
        if s == ms{
            true
        }else{
            false
        }
    }
}