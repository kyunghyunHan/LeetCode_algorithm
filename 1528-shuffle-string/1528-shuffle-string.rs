impl Solution {
pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let a = s.chars().collect::<Vec<char>>();
    let mut result = vec![' '; indices.len()];
    for i in 0..indices.len() {
        result[indices[i] as usize] = a[i];
    }
    result.into_iter().collect()
}
}