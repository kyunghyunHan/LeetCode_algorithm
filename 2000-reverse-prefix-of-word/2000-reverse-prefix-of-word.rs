impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
         let word = word.chars().collect::<Vec<char>>();
    let ch_nth = word.iter().position(|x| x == &ch).unwrap_or_else(|| 0);

    if ch_nth == 0 {
        println!("{:?}",word.iter().collect::<String>());
        return word.iter().collect::<String>();
         
    }
    let mut result = Vec::new();
    for i in (0..=ch_nth).rev() {
        result.push(word[i]);
    }
    for i in ch_nth + 1..word.len() {
        result.push(word[i]);
    }
    let result = result.iter().collect::<String>();
    println!("{}",result);
    result
    }
}