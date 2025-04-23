impl Solution {
   pub fn length_of_last_word(s: String) -> i32 {

    let last_string = s.split_whitespace().map(|x| x).collect::<Vec<&str>>();

    let result = *last_string.last().unwrap();
    let lens = result.chars().collect::<Vec<char>>().len();
    lens as i32
}

}