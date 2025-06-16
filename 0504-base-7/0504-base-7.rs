impl Solution {
 pub fn convert_to_base7(mut num: i32) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let sign = if num < 0 {
        "-".to_string()
    } else {
        "".to_string()
    };
    num = num.abs();
    let mut res = "".to_string();

    while num > 0 {
        res = (num % 7).to_string() + &res;
        num = (num / 7);
    }

    return sign + &res.to_string();
}
}