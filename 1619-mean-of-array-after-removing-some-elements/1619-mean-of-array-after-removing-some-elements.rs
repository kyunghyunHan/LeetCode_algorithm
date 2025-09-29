impl Solution {
   pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        arr.sort(); 
        let n = arr.len();
        let k = n * 5 / 100;

        let sum: f64 = arr[k..n - k].iter().map(|&x| x as f64).sum();
        sum / (n - 2 * k) as f64
    }
}