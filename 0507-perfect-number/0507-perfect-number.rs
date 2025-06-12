impl Solution {
pub fn check_perfect_number(num: i32) -> bool {
    if num < 2 {
        return false;
    }

    let mut s = 1;
    let mut i = 2;

    while i * i <= num {
        if num % i == 0 {
            s += i;
            if i * i != num {
                s += num / i;
            }
        }
        i += 1;
    }
    return s == num;
}

}