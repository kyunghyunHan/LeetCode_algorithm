impl Solution {
    pub fn sum_of_digits(nums: Vec<i32>) -> i32 {
if let Some(num) = nums.iter().min(){
    let mut result = 0;
    let num = num.to_string();
    for n in num.chars(){
        result+= n.to_string().parse::<i32>().unwrap();
    }
    if result  % 2 !=0 {
        0
    }else{
       1
    }

}else{
    0
}    
    }
}