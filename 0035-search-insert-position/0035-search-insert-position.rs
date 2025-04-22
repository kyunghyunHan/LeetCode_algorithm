impl Solution {
 pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
     let mut new_nums = nums;
     new_nums.push(target);
     new_nums.sort_by(|a,b|a.cmp(b));
     let result = new_nums.iter().position(|&x| x == target).unwrap() as i32;
     result
}
}