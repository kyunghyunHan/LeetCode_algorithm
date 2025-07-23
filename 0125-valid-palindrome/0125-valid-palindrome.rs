impl Solution {
    
pub fn is_palindrome(s: String) -> bool {
    let s: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    if s == s.clone().into_iter().rev().collect::<Vec<char>>() {
        return true;
    } else {
        return false;
    }
}
}