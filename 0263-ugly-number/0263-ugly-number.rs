impl Solution {
  pub fn is_ugly(n: i32) -> bool {
    if n <= 0 {
        return false;
    }

    let mut n = n;

    for factor in [2, 3, 5] {
        n = Self::keep_dividing_when_divisible(n, factor);
    }

    return n == 1;
}

fn keep_dividing_when_divisible(mut dividend: i32, divisor: i32) -> i32 {
    while dividend % divisor == 0 {
        dividend /= divisor;
    }
    return dividend;
}

}