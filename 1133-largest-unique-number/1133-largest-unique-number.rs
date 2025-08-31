use std::collections::HashMap;
impl Solution {
    pub fn largest_unique_number(mut nums: Vec<i32>) -> i32 {
            let mut freq = HashMap::new();
    for &n in &nums {
        *freq.entry(n).or_insert(0) += 1;
    }
    let unique: Vec<_> = freq
        .into_iter()
        .filter(|&(_, count)| count == 1)
        .map(|(num, _)| num)
        .collect();

    if let Some(data)=   unique.iter().max(){
        *data
    }else{
        -1
    }
    }
}