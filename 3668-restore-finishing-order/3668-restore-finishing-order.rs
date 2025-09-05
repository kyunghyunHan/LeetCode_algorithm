impl Solution {
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        let mut result =Vec::new();
        for i in order{
          if  friends.contains(&i){
            result.push(i)
          }
        }
        result
    }
}