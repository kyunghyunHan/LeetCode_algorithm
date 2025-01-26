impl Solution {
    pub fn score_of_string(s: String) -> i32 {
    let v = s.chars().collect::<Vec<char>>();
    let mut result = 0;

    for i in 1..v.len() {
        if v[i] < v[i - 1] {
            result += v[i - 1] as i32 - v[i] as i32;
        } else if v[i] > v[i - 1] {
            result += v[i] as i32 - v[i - 1] as i32;
        } else {
            result += 0;
        }
    }
    result
}

}