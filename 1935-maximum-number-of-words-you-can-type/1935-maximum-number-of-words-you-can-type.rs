impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
       let text = text.split(" ").map(|x|x.to_string()).collect::<Vec<String>>();
              let mut count = text.len();  

        let broken_letters = broken_letters.chars().collect::<Vec<char>>();

       for i in 0..text.len(){
        for j in 0..broken_letters.len(){
            if text[i].contains(&broken_letters[j].to_string()){
             count-=1;
             break;
            }
        }
       }
       count as i32
       
    }
}