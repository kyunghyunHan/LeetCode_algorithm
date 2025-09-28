impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let mut count = 0;
        for item in items {
            match rule_key.as_str(){
                "type" =>{
                    if item[0] == rule_value{
                        count+=1;
                    }
                }
                "color" =>{
                    if item[1] == rule_value{
                        count+=1;
                    }
                }
                _=>{
                    if item[2] == rule_value{
                        count+=1;
                    }
                }
            }
        }
        count
    }
}