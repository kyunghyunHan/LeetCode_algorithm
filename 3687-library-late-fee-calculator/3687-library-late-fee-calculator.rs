impl Solution {
    pub fn late_fee(days_late: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..days_late.len(){
           if days_late[i] == 1 {
            result+=1;
           }else if days_late[i] >= 2 && days_late[i] <= 5{
            result+=2 * days_late[i];
           }else if days_late[i] > 5 {
            result+=3* days_late[i];
           }
        } 
        result
    }
}