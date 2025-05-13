impl Solution {
  pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut v = Vec::new();

    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
            v.push("FizzBuzz".to_string());
        } else if i % 3 == 0 {
            v.push("Fizz".to_string());
        } else if i % 5 == 0 {
            v.push("Buzz".to_string());
        } else {
            v.push(i.to_string());
        }
    }
    v
}

}