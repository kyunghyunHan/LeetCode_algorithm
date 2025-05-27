impl Solution {
   pub fn arrange_coins(n: i32) -> i32 {
    // let mut dp: Vec<i32> = vec![0; 1000000000];
    let mut cnt = 0;
    let mut a = n;
    for i in 1..=n {
        a -= i;
        cnt += 1;
        if a <= i {
            break;
        }
    }
    cnt
}
}