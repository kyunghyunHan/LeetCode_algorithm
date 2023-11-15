impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
    let s= n.to_string();
    let  nums:Vec<i32>= s.chars().map(|x|x.to_string().parse().unwrap()).collect::<Vec<i32>>();

    let sum:i32= nums.clone().into_iter().sum();
    let mul:i32= nums.clone().into_iter().product();
        
     mul-sum

        
    }
}