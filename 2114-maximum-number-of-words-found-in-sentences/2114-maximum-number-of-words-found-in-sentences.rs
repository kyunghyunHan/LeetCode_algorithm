impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut count = 0;
        for sentence in sentences {
            let l = sentence.split(" ").collect::<Vec<&str>>().len();
            if l > count {
                count  = l;
            }
        }
        count as i32
    }
}