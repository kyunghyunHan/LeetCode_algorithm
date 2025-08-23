impl Solution {
    pub fn reverse_words(s: String) -> String {
        
    let s = s
        .split_whitespace()
        .map(|x| {
            // let x = x.to_string();
            let mut x = x.chars().collect::<Vec<char>>();
            x.reverse();
            let s: String = x.iter().collect();
            s
        })
        .collect::<Vec<_>>();
    let s: String = s.join(" ");
    s
    }
}