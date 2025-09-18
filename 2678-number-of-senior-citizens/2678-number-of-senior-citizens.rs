impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
          let mut num = "".to_string();
    let mut count = 0;
    for detail in details {
        let detail = detail.chars().collect::<Vec<char>>();
        for c in 0..detail.len() {
            if detail[c].is_alphabetic() {
                num = detail[c + 1].to_string() + &detail[c + 2].to_string();
                let result = num.parse::<i32>().unwrap();

                if result > 60{
                    count+=1;
                }
            }
        }
    }
    println!("{}", count);
    count
    }
}