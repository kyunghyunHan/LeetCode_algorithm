impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut i = 0;

        for s in commands{
            match s.as_str(){
                "RIGHT" => {
                     i+=1;
                }
                     "LEFT" => {
                i-=1;
                }
                     "UP" => {
                    i -= n;
                }
                _=>{
                    i += n;

                }
            }
        }
        i

        
    }
}