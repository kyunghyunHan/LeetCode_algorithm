impl Solution {
   pub fn maximum69_number(num: i32) -> i32 {
    let mut chars: Vec<char> = num.to_string().chars().collect();
    for ch in chars.iter_mut() {
        if *ch == '6' {
            *ch = '9';
            break;
        }
    }
    chars.into_iter().collect::<String>().parse::<i32>().unwrap()
}
}