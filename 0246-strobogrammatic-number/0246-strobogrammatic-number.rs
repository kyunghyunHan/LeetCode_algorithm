impl Solution {
   pub fn is_strobogrammatic(num: String) -> bool {
    let mut rotated = String::with_capacity(num.len());

    for c in num.chars().rev() {
        match c {
            '0' | '1' | '8' => rotated.push(c),
            '6' => rotated.push('9'),
            '9' => rotated.push('6'),
            _ => return false,
        }
    }

    rotated == num
}
}