impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        use std::collections::HashSet;

        let row1: HashSet<char> = "qwertyuiop".chars().collect();
        let row2: HashSet<char> = "asdfghjkl".chars().collect();
        let row3: HashSet<char> = "zxcvbnm".chars().collect();

        let mut result = Vec::new();

        for word in words {
            let lower = word.to_lowercase();
            let chars: Vec<char> = lower.chars().collect();

            let target_row = if row1.contains(&chars[0]) {
                &row1
            } else if row2.contains(&chars[0]) {
                &row2
            } else {
                &row3
            };

            if chars.iter().all(|c| target_row.contains(c)) {
                result.push(word);
            }
        }

        result
    }
}