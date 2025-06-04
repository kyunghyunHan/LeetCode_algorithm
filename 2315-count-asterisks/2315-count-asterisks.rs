impl Solution {
  pub fn count_asterisks(s: String) -> i32 {
    let mut res = 0;
    let mut bars = 0;

    for i in s.chars() {
        if i == '*' && bars % 2 == 0 {
            res += 1;
        }
        if i == '|' {
            bars += 1;
        }
    }
    res
}
}