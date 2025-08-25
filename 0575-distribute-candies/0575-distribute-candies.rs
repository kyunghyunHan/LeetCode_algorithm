use std::collections::HashSet;
impl Solution {
    pub fn distribute_candies(mut candy_type: Vec<i32>) -> i32 {
      let num_len = candy_type.len();

    let unique: HashSet<i32> = candy_type.iter().copied().collect::<HashSet<i32>>();
    let unique_len = unique.len();
    

    if unique_len > num_len / 2 {
        return (num_len / 2) as i32 ;
    } else {
        return unique_len as i32
    }
    }
}