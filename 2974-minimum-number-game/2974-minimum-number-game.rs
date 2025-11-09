impl Solution {
    pub fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let mut alice = vec![];
        let mut bob = vec![];
        nums.sort_by(|a,b|a.cmp(&b));

        
        for i in 0..nums.len(){
            if i % 2 ==0{
              alice.push(nums[i]);
            }else{
              bob.push(nums[i]);
            }
        }

         for i in 0..alice.len(){
            result.push(bob[i]);

            result.push(alice[i]);

         
        }
        result
    }
}