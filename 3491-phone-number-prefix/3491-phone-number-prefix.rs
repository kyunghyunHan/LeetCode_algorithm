impl Solution {
    pub fn phone_prefix(numbers: Vec<String>) -> bool {
        let mut result = true;

        for i in 0..numbers.len(){
            for j in 0..numbers.len(){
                 if i!=j  && numbers[j].starts_with(&numbers[i]){
                    result= false;
                 }
            }
        }
        result
    }
}