impl Solution {
   pub fn arrays_intersection(arr1: Vec<i32>, arr2: Vec<i32>, arr3: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for i in arr1 {
        if arr2.contains(&i) && arr3.contains(&i) {
            result.push(i);
        }
    }
    result
}
}