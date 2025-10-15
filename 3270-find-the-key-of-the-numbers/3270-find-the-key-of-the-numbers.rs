impl Solution {
    pub fn generate_key(mut num1: i32,mut  num2: i32, mut num3: i32) -> i32 {
        let (mut x,mut ans) =(1,0);
        while num1 > 0 && num2 > 0 && num3 > 0 {
        let d1 = num1 % 10;
        let d2 = num2 % 10;
        let d3 = num3 % 10;

        let min_digit = d1.min(d2).min(d3);

        ans += x * min_digit;
        num1 /= 10;
        num2 /= 10;
        num3 /= 10;
        x *= 10;
    }

    ans
    }
}