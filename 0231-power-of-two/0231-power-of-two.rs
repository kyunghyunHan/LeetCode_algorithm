impl Solution {
fn is_power_of_two(mut n: i32) -> bool {
    if n <= 0 {
        return false;
    }

    while n % 2 == 0 {
        n /= 2;
        if n == 1 {
            return true;
        }
    }

    n == 1
}

}