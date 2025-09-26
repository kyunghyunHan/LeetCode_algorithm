use std::collections::HashMap;

impl Solution {
 pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    let mut freq = HashMap::new();
    for w in s1.split_whitespace().chain(s2.split_whitespace()) {
        *freq.entry(w.to_string()).or_insert(0) += 1;
    }
    freq.into_iter()
        .filter_map(|(w,c)| if c==1 {Some(w)} else {None})
        .collect()
}
}