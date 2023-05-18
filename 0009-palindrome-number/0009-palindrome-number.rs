impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
         if x < 0 {
        return false;
    }

    let mut reverse = 0;
    let mut temp = x;

    while temp > 0 {
        reverse = reverse * 10 + temp % 10;
        temp /= 10;
    }

    reverse == x
    }
}