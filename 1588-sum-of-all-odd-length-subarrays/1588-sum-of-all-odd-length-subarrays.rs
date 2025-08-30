impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in &arr{
            println!("{}",i);
            result+=*i;
        };
        for i in 0..arr.len(){
            for j in i..arr.len(){
                if i == j  ||( j - i) % 2 != 0 || (j-i)==1 {
                    continue;
                }

                let sums: i32 = arr[i..=j].iter().sum();
                println!("{}",sums);
                result += sums;
            }
        }
        result
    }
}