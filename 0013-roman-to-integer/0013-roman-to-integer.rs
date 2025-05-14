impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
    let mut cnt = 0;
    let v = s.chars().collect::<Vec<char>>();
    for i in 0..v.len() {
        println!("{}",v[i]);
        match v[i] {
            'I' => cnt += 1,
            'V' => {
                if i > 0 && v[i - 1] == 'I' {
                    cnt -= 1;
                    cnt += 4
                } else {
                    cnt += 5
                }
            }
            'X' => {
                if i > 0 && v[i - 1] == 'I' {
                    cnt -= 1;
                    cnt += 9
                } else {
                    cnt += 10
                }
            }
            'L' => {
                if i > 0 && v[i - 1] == 'X' {
                    cnt -= 10;
                    cnt += 40
                } else {
                    cnt += 50
                }
            }
            'C' => {
                if i > 0 && v[i - 1] == 'X' {
                    cnt -= 10;
                    cnt += 90
                } else {
                    cnt += 100
                }
            }
            'D' => {
                if i > 0 && v[i - 1] == 'C' {
                    cnt -= 100;
                    cnt += 400
                } else {
                    cnt += 500
                }
            }
            'M' => {
                if i > 0 && v[i - 1] == 'C' {
                    cnt -= 100;
                    cnt += 900
                } else {
                    cnt += 1000
                }
            }
            _ => {}
        }
    }
    cnt
}

}