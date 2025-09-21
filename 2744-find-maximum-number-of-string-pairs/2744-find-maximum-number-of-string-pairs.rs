impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
       let mut count = 0;
        for i in 0..words.len()  {
            for j in i+1..words.len(){
                // let s1 = words[i].chars().collect::<Vec<char>>();
                // let s2 = words[j].chars().collect::<Vec<char>>();
              if words[i] == words[j] || words[i] == words[j].chars().rev(). collect::<String>() {
                     count += 1;
                 }

            }
        }
        count
        
    }
}