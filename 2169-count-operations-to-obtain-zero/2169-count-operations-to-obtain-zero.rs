impl Solution {
    pub fn count_operations(mut num1: i32, mut  num2: i32) -> i32 {
        let mut count   = 0; 
        if num1 ==0 && num2 ==0 {
            return count
        }else if (num1==0 && num2 > 0)||(num2==0 && num1 > 0){
            return count

        }
        loop{
            count+=1;

            if num1 > num2 {
                num1 -= num2;
            }else if num1  < num2{
                num2 -= num1;
            }else{
                break
            }
        }
        count
    }
}