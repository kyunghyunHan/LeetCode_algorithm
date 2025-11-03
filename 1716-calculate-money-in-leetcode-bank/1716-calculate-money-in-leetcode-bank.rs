impl Solution {
    pub fn total_money(mut n: i32) -> i32 {
     let mut a = 0;
     let mut monday =1;

     while n> 0{
        for day in 0..n.min(7){
            a+=(monday+ day);
        }
        n-=7;
        monday+=1
     }
     a
    }
}