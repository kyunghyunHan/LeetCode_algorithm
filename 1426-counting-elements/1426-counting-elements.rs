impl Solution {
    pub fn count_elements(mut arr: Vec<i32>) -> i32 {
        let mut count= 0;
        for i in 0..arr.len(){
            if arr.contains(&(arr[i]+1)){
                count+=1;
            }
        }
        count
    }
}