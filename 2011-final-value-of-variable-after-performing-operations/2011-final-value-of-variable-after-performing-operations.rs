impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut count = 0;
        for s in operations{
            match s.as_str(){
                "X++" | "++X" =>{
                    count+=1;
                }
                "--X"|"X--"=>{
                    count-=1
                }
                _=>{}
            }
        }
        count
        
    }
}