impl Solution {
pub fn reverse_only_letters(s: String) -> String {
    let mut letters: Vec<char> = s.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    let mut result = String::new();

    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            result.push(letters.pop().unwrap());
        } else {
            result.push(c);
        }
    }

    result
}
}