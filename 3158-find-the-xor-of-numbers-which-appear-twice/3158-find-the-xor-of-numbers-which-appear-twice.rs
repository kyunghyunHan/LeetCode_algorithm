impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut v = vec![0;51];
        for num in nums{
            v[num as usize]+=1;
        }
        println!("{:?}",v);

        let mut result = vec![];
        for i in 0..v.len(){
            if v[i] >= 2{
                result.push(i);
            }
        }
        if result.len() ==0 {
            0
        }else if result.len()==1{
            (result[0]) as i32
        }else {
            let mut re =0;
            for  i in result {
                re ^=i;
                
            }
            re as i32
        }
        
    }
}