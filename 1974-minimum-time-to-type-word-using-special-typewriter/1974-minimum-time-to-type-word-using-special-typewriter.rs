impl Solution {
   pub fn min_time_to_type(word: String) -> i32 {
    let n = word.len();
    let mut time = n as i32;
    let mut s = word.chars().collect::<Vec<char>>();
    for i in 1..n {
        let mov =( s[i as usize - 1] as i32 - s[i as usize] as i32).abs();
        time += mov.min(26 - mov);
    }
    time + (s[0] as i32 - 'a' as i32).min('z' as i32 - s[0] as i32 + 1)
}

}