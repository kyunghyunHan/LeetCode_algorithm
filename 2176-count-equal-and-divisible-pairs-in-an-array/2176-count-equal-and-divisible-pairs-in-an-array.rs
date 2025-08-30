impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut count  = 0;
        for  i in 0..nums.len(){
            for j in i..nums.len(){
                if nums[i] ==nums[j]{
                    if  i !=j && (i * j) as i32 % k ==0{
                        println!("{}{}",i,j);
                        println!("{}",count);
                        count+=1;
                        continue;
                
                    }
                }
            }
        }
        count
    }
}