impl Solution {
pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    let mut re = true;

    for i in 0..arr.len() {
        if arr[i] == 0 && re == true {
            arr.insert(i, 0);
            arr.pop();
            re = false;
        } else {
            re = true;
        }
    }
}
}