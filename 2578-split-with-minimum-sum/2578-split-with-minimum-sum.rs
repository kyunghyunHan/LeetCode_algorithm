impl Solution {
    pub fn split_num(num: i32) -> i32 {
        let mut nums = num.to_string().chars().map(|x|x as u32 - 48).collect::<Vec<u32>>(); 
        nums.sort_by(|a,b|a.cmp(&b));

        
        let mut nums1 = "".to_string();
        let mut nums2 = "".to_string(); 

        for i in 0..nums.len(){
            if i % 2==0{
                nums2+=&nums[i].to_string();
            }else{
               nums1+=&nums[i].to_string();
            }

        }
        println!("{:?}",nums);
        nums1.parse::<i32>().unwrap() + nums2.parse::<i32>().unwrap()
    }
}