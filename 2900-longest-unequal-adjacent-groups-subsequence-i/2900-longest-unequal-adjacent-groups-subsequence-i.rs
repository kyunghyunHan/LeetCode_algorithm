impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
    let mut dp = Vec::new();

    dp.push(words[0].clone());

    for i in 1..groups.len() {
        if groups[i - 1] != groups[i] {
            dp.push(words[i].clone());
        }
    }
    dp
}
}