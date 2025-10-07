impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut result = vec![];

        for i in left..=right {
            let digits = i
                .to_string()
                .chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>();

            let mut valid = true;
            for d in digits {
                if d == 0 || i % d != 0 {
                    valid = false;
                    break;
                }
            }

            if valid {
                result.push(i);
            }
        }

        result
    }
}