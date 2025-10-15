impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut result = vec![];
        for i in 0..words.len(){

          for c in words[i].chars(){
               if c == x{
                  result.push(i as i32);
                   break;
               }
               
          }
        }
        result
    }
}