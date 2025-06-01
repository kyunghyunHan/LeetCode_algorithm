use std::collections::BTreeMap;

impl Solution {
pub fn sort_sentence(s: String) -> String {
    let mut map = BTreeMap::new();

    for word in s.split_whitespace() {
        let len = word.len();
        let pos = word[len - 1..].parse::<usize>().unwrap();
        let word_part = &word[..len - 1];
        map.insert(pos, word_part);
    }

    map.values().cloned().collect::<Vec<&str>>().join(" ")
}
}