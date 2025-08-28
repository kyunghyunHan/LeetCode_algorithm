impl Solution {
    pub fn generate_the_string(n: i32) -> String {
          if n % 2 == 0 {
        let s = format!("{}{}", "a".repeat(n  as usize- 1), "b");
        println!("{}", s);
        s
    } else {
        let s = format!("{}", "a".repeat(n as usize));
        println!("{}", s);
        s
    }
    }
}