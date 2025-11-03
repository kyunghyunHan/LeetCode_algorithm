impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let min = nums.iter().min().unwrap();
        let max = nums.iter().max().unwrap();

        let result = gcd(*min,*max);
        result
    }
}
fn gcd(a:i32,b:i32)->i32{
    if a==0 {
        return b
    }
    gcd(b%a,a)
}