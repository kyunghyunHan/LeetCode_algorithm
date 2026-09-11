impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut ans = true;
        let mut five =0;
        let mut ten =0;

        for i in 0..bills.len(){
            if bills[i]==5{
                five+=1;
            }else if bills[i]==10{
                ten+=1;
                five-=1;
            }else if bills[i]==20{
               if ten>=1&&five>=1{
                  ten-=1;
                  five-=1;
               }else if five>=3{
                  five-=3;
               }else{
                ans =false;
               }
            }
        
        }
        if five <0 || ten < 0{
            ans = false;
        }
        ans
    }
}