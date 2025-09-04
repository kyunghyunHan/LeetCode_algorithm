impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
            let mut result = vec![];
    for i in 0..words.len() {
        if words[i].contains(&separator.to_string()) {
            println!("{}", true);
            let s = words[i].replace(separator, " ");
            for s in s.split(" ") {
                if s!=""{
                result.push(s.to_string());

                }
            }
        } else {
            result.push(words[i].to_string());
            continue;
        }
    }
    println!("{:?}", result);
    result
    }
}